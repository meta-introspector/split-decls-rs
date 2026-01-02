mkuse!{use crate :: cell :: UnsafeCell ;}
mkuse!{use crate :: mem :: ManuallyDrop ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicUsize , Ordering } ;}
mkuse!{use crate :: sys :: pal :: itron :: error :: { ItronError , expect_success , expect_success_aborting } ;}
mkuse!{use crate :: sys :: pal :: itron :: time :: dur2reltims ;}
mkuse!{use crate :: sys :: pal :: itron :: { abi , task } ;}
mkuse!{use crate :: time :: Duration ;}
mkuse!{use crate :: { hint , io } ;}
mkitem!{mkstruct!{pub struct Thread { p_inner : NonNull < ThreadInner > , # [doc = " The ID of the underlying task."] task : abi :: ID , }}}
mkitem!{mkimpl!{unsafe impl Send for Thread { }}}
mkitem!{mkimpl!{unsafe impl Sync for Thread { }}}
mkitem!{mkstruct!{# [doc = " State data shared between a parent thread and child thread. It's dropped on"] # [doc = " a transition to one of the final states."] struct ThreadInner { # [doc = " This field is used on thread creation to pass a closure from"] # [doc = " `Thread::new` to the created task."] start : UnsafeCell < ManuallyDrop < Box < dyn FnOnce () > > > , # [doc = " A state machine. Each transition is annotated with `[...]` in the"] # [doc = " source code."] # [doc = ""] # [doc = " ```text"] # [doc = ""] # [doc = "    <P>: parent, <C>: child, (?): don't-care"] # [doc = ""] # [doc = "       DETACHED (-1)  -------------------->  EXITED (?)"] # [doc = "                        <C>finish/exd_tsk"] # [doc = "          ^"] # [doc = "          |"] # [doc = "          | <P>detach"] # [doc = "          |"] # [doc = ""] # [doc = "       INIT (0)  ----------------------->  FINISHED (-1)"] # [doc = "                        <C>finish"] # [doc = "          |                                    |"] # [doc = "          | <P>join/slp_tsk                    | <P>join/del_tsk"] # [doc = "          |                                    | <P>detach/del_tsk"] # [doc = "          v                                    v"] # [doc = ""] # [doc = "       JOINING                              JOINED (?)"] # [doc = "     (parent_tid)"] # [doc = "                                            ^"] # [doc = "             \\                             /"] # [doc = "              \\  <C>finish/wup_tsk        / <P>slp_tsk-complete/ter_tsk"] # [doc = "               \\                         /                      & del_tsk"] # [doc = "                \\                       /"] # [doc = "                 '--> JOIN_FINALIZE ---'"] # [doc = "                          (-1)"] # [doc = ""] lifecycle : Atomic < usize > , }}}
mkitem!{mkimpl!{unsafe impl Sync for ThreadInner { }}}
mkitem!{const LIFECYCLE_INIT : usize = 0 ;}
mkitem!{const LIFECYCLE_FINISHED : usize = usize :: MAX ;}
mkitem!{const LIFECYCLE_DETACHED : usize = usize :: MAX ;}
mkitem!{const LIFECYCLE_JOIN_FINALIZE : usize = usize :: MAX ;}
mkitem!{const LIFECYCLE_DETACHED_OR_JOINED : usize = usize :: MAX ;}
mkitem!{const LIFECYCLE_EXITED_OR_FINISHED_OR_JOIN_FINALIZE : usize = usize :: MAX ;}
mkitem!{pub const DEFAULT_MIN_STACK_SIZE : usize = 0x4000 * size_of :: < usize > () ;}
mkitem!{mkimpl!{impl Thread { # [doc = " # Safety"] # [doc = ""] # [doc = " See `thread::Builder::spawn_unchecked` for safety requirements."] pub unsafe fn new (stack : usize , _name : Option < & str > , p : Box < dyn FnOnce () > ,) -> io :: Result < Thread > { let inner = Box :: new (ThreadInner { start : UnsafeCell :: new (ManuallyDrop :: new (p)) , lifecycle : AtomicUsize :: new (LIFECYCLE_INIT) , }) ; unsafe extern "C" fn trampoline (exinf : isize) { let p_inner : * mut ThreadInner = crate :: ptr :: with_exposed_provenance_mut (exinf as usize) ; let inner = unsafe { & * p_inner } ; let p = unsafe { ManuallyDrop :: take (& mut * inner . start . get ()) } ; p () ; let _ = unsafe { abi :: unl_cpu () } ; let _ = unsafe { abi :: ena_dsp () } ; unsafe { crate :: sys :: thread_local :: destructors :: run () } ; let old_lifecycle = inner . lifecycle . swap (LIFECYCLE_EXITED_OR_FINISHED_OR_JOIN_FINALIZE , Ordering :: AcqRel) ; match old_lifecycle { LIFECYCLE_DETACHED => { let _ = unsafe { Box :: from_raw (p_inner) } ; unsafe { terminate_and_delete_current_task () } ; } LIFECYCLE_INIT => { } parent_tid => { expect_success (unsafe { let mut er = abi :: wup_tsk (parent_tid as _) ; if er == abi :: E_QOVR { er = abi :: E_OK ; } er } , & "wup_tsk" ,) ; } } } let p_inner = unsafe { NonNull :: new_unchecked (Box :: into_raw (inner)) } ; let new_task = ItronError :: err_if_negative (unsafe { abi :: acre_tsk (& abi :: T_CTSK { tskatr : abi :: TA_ACT , exinf : p_inner . as_ptr () . expose_provenance () as abi :: EXINF , task : Some (trampoline) , itskpri : abi :: TPRI_SELF , stksz : stack , stk : crate :: ptr :: null_mut () , }) }) . map_err (| e | e . as_io_error ()) ? ; Ok (Self { p_inner , task : new_task }) } pub fn join (self) { let inner = unsafe { self . p_inner . as_ref () } ; let current_task = task :: current_task_id_aborting () ; debug_assert ! (usize :: try_from (current_task) . is_ok ()) ; debug_assert_ne ! (current_task as usize , LIFECYCLE_INIT) ; debug_assert_ne ! (current_task as usize , LIFECYCLE_DETACHED) ; let current_task = current_task as usize ; match inner . lifecycle . swap (current_task , Ordering :: AcqRel) { LIFECYCLE_INIT => { loop { expect_success_aborting (unsafe { abi :: slp_tsk () } , & "slp_tsk") ; if inner . lifecycle . load (Ordering :: Acquire) == LIFECYCLE_JOIN_FINALIZE { break ; } } } LIFECYCLE_FINISHED => { } _ => unsafe { hint :: unreachable_unchecked () } , } unsafe { terminate_and_delete_task (self . task) } ; let _inner = unsafe { Box :: from_raw (self . p_inner . as_ptr ()) } ; crate :: mem :: forget (self) ; } }}}
mkitem!{mkimpl!{impl Drop for Thread { fn drop (& mut self) { let inner = unsafe { self . p_inner . as_ref () } ; match inner . lifecycle . swap (LIFECYCLE_DETACHED_OR_JOINED , Ordering :: AcqRel) { LIFECYCLE_INIT => { } LIFECYCLE_FINISHED => { unsafe { terminate_and_delete_task (self . task) } ; let _ = unsafe { Box :: from_raw (self . p_inner . as_ptr ()) } ; } _ => unsafe { hint :: unreachable_unchecked () } , } } }}}

macro_rules! terminate_and_delete_task_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function terminate_and_delete_task in module {}", module_path!());
    };
}

mkfn!{
    terminate_and_delete_task_introspect!();
    # [doc = " Terminates and deletes the specified task."] # [doc = ""] # [doc = " This function will abort if `deleted_task` refers to the calling task."] # [doc = ""] # [doc = " It is assumed that the specified task is solely managed by the caller -"] # [doc = " i.e., other threads must not \"resuscitate\" the specified task or delete it"] # [doc = " prematurely while this function is still in progress. It is allowed for the"] # [doc = " specified task to exit by its own."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The task must be safe to terminate. This is in general not true"] # [doc = " because there might be pinned references to the task's stack."] unsafe fn terminate_and_delete_task (deleted_task : abi :: ID) { match unsafe { abi :: ter_tsk (deleted_task) } { abi :: E_OBJ => { } er => { expect_success_aborting (er , & "ter_tsk") ; } } expect_success_aborting (unsafe { abi :: del_tsk (deleted_task) } , & "del_tsk") ; }
}

macro_rules! terminate_and_delete_current_task_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function terminate_and_delete_current_task in module {}", module_path!());
    };
}

mkfn!{
    terminate_and_delete_current_task_introspect!();
    # [doc = " Terminates and deletes the calling task."] # [doc = ""] # [doc = " Atomicity is not required - i.e., it can be assumed that other threads won't"] # [doc = " `ter_tsk` the calling task while this function is still in progress. (This"] # [doc = " property makes it easy to implement this operation on μITRON-derived kernels"] # [doc = " that don't support `exd_tsk`.)"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The task must be safe to terminate. This is in general not true"] # [doc = " because there might be pinned references to the task's stack."] unsafe fn terminate_and_delete_current_task () -> ! { expect_success_aborting (unsafe { abi :: exd_tsk () } , & "exd_tsk") ; unsafe { crate :: hint :: unreachable_unchecked () } ; }
}

macro_rules! yield_now_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function yield_now in module {}", module_path!());
    };
}

mkfn!{
    yield_now_introspect!();
    pub fn yield_now () { expect_success (unsafe { abi :: rot_rdq (abi :: TPRI_SELF) } , & "rot_rdq") ; }
}

macro_rules! sleep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep in module {}", module_path!());
    };
}

mkfn!{
    sleep_introspect!();
    pub fn sleep (dur : Duration) { for timeout in dur2reltims (dur) { expect_success (unsafe { abi :: dly_tsk (timeout) } , & "dly_tsk") ; } }
}