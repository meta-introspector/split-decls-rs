mkuse!{use super :: { Thread , ThreadId , imp } ;}
mkuse!{use crate :: mem :: ManuallyDrop ;}
mkuse!{use crate :: ptr ;}
mkuse!{use crate :: sys :: thread_local :: local_pointer ;}
mkitem!{const NONE : * mut () = ptr :: null_mut () ;}
mkitem!{const BUSY : * mut () = ptr :: without_provenance_mut (1) ;}
mkitem!{const DESTROYED : * mut () = ptr :: without_provenance_mut (2) ;}
mkitem!{local_pointer ! { static CURRENT ; }}
mkmod!{id, { 
                getname!(id);
                getsrc!(id);
                getpath!(id);
                get_deps!(id);
                get_crates!(id);
                mkinclude!(id);
                mkuse!{use super :: * ;}
mkitem!{cfg_select ! { target_thread_local => { use crate :: cell :: Cell ; # [thread_local] static ID : Cell < Option < ThreadId >> = Cell :: new (None) ; pub (super) const CHEAP : bool = true ; pub (crate) fn get () -> Option < ThreadId > { ID . get () } pub (super) fn set (id : ThreadId) { ID . set (Some (id)) } } target_pointer_width = "16" => { local_pointer ! { static ID0 ; static ID16 ; static ID32 ; static ID48 ; } pub (super) const CHEAP : bool = false ; pub (crate) fn get () -> Option < ThreadId > { let id0 = ID0 . get () . addr () as u64 ; let id16 = ID16 . get () . addr () as u64 ; let id32 = ID32 . get () . addr () as u64 ; let id48 = ID48 . get () . addr () as u64 ; ThreadId :: from_u64 ((id48 << 48) + (id32 << 32) + (id16 << 16) + id0) } pub (super) fn set (id : ThreadId) { let val = id . as_u64 () . get () ; ID0 . set (ptr :: without_provenance_mut (val as usize)) ; ID16 . set (ptr :: without_provenance_mut ((val >> 16) as usize)) ; ID32 . set (ptr :: without_provenance_mut ((val >> 32) as usize)) ; ID48 . set (ptr :: without_provenance_mut ((val >> 48) as usize)) ; } } target_pointer_width = "32" => { local_pointer ! { static ID0 ; static ID32 ; } pub (super) const CHEAP : bool = false ; pub (crate) fn get () -> Option < ThreadId > { let id0 = ID0 . get () . addr () as u64 ; let id32 = ID32 . get () . addr () as u64 ; ThreadId :: from_u64 ((id32 << 32) + id0) } pub (super) fn set (id : ThreadId) { let val = id . as_u64 () . get () ; ID0 . set (ptr :: without_provenance_mut (val as usize)) ; ID32 . set (ptr :: without_provenance_mut ((val >> 32) as usize)) ; } } _ => { local_pointer ! { static ID ; } pub (super) const CHEAP : bool = true ; pub (crate) fn get () -> Option < ThreadId > { let id = ID . get () . addr () as u64 ; ThreadId :: from_u64 (id) } pub (super) fn set (id : ThreadId) { let val = id . as_u64 () . get () ; ID . set (ptr :: without_provenance_mut (val as usize)) ; } } }}

macro_rules! get_or_init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_or_init in module {}", module_path!());
    };
}

mkfn!{
    get_or_init_introspect!();
    # [inline] pub (super) fn get_or_init () -> ThreadId { get () . unwrap_or_else (# [cold] | | { let id = ThreadId :: new () ; id :: set (id) ; id } ,) }
} 
            }}

macro_rules! set_current_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_current in module {}", module_path!());
    };
}

mkfn!{
    set_current_introspect!();
    # [doc = " Tries to set the thread handle for the current thread. Fails if a handle was"] # [doc = " already set or if the thread ID of `thread` would change an already-set ID."] pub (super) fn set_current (thread : Thread) -> Result < () , Thread > { if CURRENT . get () != NONE { return Err (thread) ; } match id :: get () { Some (id) if id == thread . id () => { } None => id :: set (thread . id ()) , _ => return Err (thread) , } crate :: sys :: thread_local :: guard :: enable () ; CURRENT . set (thread . into_raw () . cast_mut ()) ; Ok (()) }
}

macro_rules! current_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_id in module {}", module_path!());
    };
}

mkfn!{
    current_id_introspect!();
    # [doc = " Gets the id of the thread that invokes it."] # [doc = ""] # [doc = " This function will always succeed, will always return the same value for"] # [doc = " one thread and is guaranteed not to call the global allocator."] # [inline] pub (crate) fn current_id () -> ThreadId { if ! id :: CHEAP { if let Some (id) = try_with_current (| t | t . map (| t | t . id ())) { return id ; } } id :: get_or_init () }
}

macro_rules! current_os_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_os_id in module {}", module_path!());
    };
}

mkfn!{
    current_os_id_introspect!();
    # [doc = " Gets the OS thread ID of the thread that invokes it, if available. If not, return the Rust"] # [doc = " thread ID."] # [doc = ""] # [doc = " We use a `u64` to all possible platform IDs without excess `cfg`; most use `int`, some use a"] # [doc = " pointer, and Apple uses `uint64_t`. This is a \"best effort\" approach for diagnostics and is"] # [doc = " allowed to fall back to a non-OS ID (such as the Rust thread ID) or a non-unique ID (such as a"] # [doc = " PID) if the thread ID cannot be retrieved."] pub (crate) fn current_os_id () -> u64 { imp :: current_os_id () . unwrap_or_else (| | current_id () . as_u64 () . get ()) }
}

macro_rules! try_with_current_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_with_current in module {}", module_path!());
    };
}

mkfn!{
    try_with_current_introspect!();
    # [doc = " Gets a reference to the handle of the thread that invokes it, if the handle"] # [doc = " has been initialized."] pub (super) fn try_with_current < F , R > (f : F) -> R where F : FnOnce (Option < & Thread >) -> R , { let current = CURRENT . get () ; if current > DESTROYED { unsafe { let current = ManuallyDrop :: new (Thread :: from_raw (current)) ; f (Some (& current)) } } else { f (None) } }
}

macro_rules! current_or_unnamed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_or_unnamed in module {}", module_path!());
    };
}

mkfn!{
    current_or_unnamed_introspect!();
    # [doc = " Gets a handle to the thread that invokes it. If the handle stored in thread-"] # [doc = " local storage was already destroyed, this creates a new unnamed temporary"] # [doc = " handle to allow thread parking in nearly all situations."] pub (crate) fn current_or_unnamed () -> Thread { let current = CURRENT . get () ; if current > DESTROYED { unsafe { let current = ManuallyDrop :: new (Thread :: from_raw (current)) ; (* current) . clone () } } else if current == DESTROYED { Thread :: new (id :: get_or_init () , None) } else { init_current (current) } }
}

macro_rules! current_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current in module {}", module_path!());
    };
}

mkfn!{
    current_introspect!();
    # [doc = " Gets a handle to the thread that invokes it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Getting a handle to the current thread with `thread::current()`:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let handler = thread::Builder::new()"] # [doc = "     .name(\"named thread\".into())"] # [doc = "     .spawn(|| {"] # [doc = "         let handle = thread::current();"] # [doc = "         assert_eq!(handle.name(), Some(\"named thread\"));"] # [doc = "     })"] # [doc = "     .unwrap();"] # [doc = ""] # [doc = " handler.join().unwrap();"] # [doc = " ```"] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] pub fn current () -> Thread { let current = CURRENT . get () ; if current > DESTROYED { unsafe { let current = ManuallyDrop :: new (Thread :: from_raw (current)) ; (* current) . clone () } } else { init_current (current) } }
}

macro_rules! init_current_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init_current in module {}", module_path!());
    };
}

mkfn!{
    init_current_introspect!();
    # [cold] fn init_current (current : * mut ()) -> Thread { if current == NONE { CURRENT . set (BUSY) ; let id = id :: get_or_init () ; let thread = Thread :: new (id , None) ; crate :: sys :: thread_local :: guard :: enable () ; CURRENT . set (thread . clone () . into_raw () . cast_mut ()) ; thread } else if current == BUSY { rtabort ! ("\n\
            Attempted to access thread-local data while allocating said data.\n\
            Do not access functions that allocate in the global allocator!\n\
            This is a bug in the global allocator.\n\
            ") } else { debug_assert_eq ! (current , DESTROYED) ; panic ! ("use of std::thread::current() is not possible after the thread's \
            local data has been destroyed") } }
}

macro_rules! drop_current_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function drop_current in module {}", module_path!());
    };
}

mkfn!{
    drop_current_introspect!();
    # [doc = " This should be run in [`crate::rt::thread_cleanup`] to reset the thread"] # [doc = " handle."] pub (crate) fn drop_current () { let current = CURRENT . get () ; if current > DESTROYED { unsafe { CURRENT . set (DESTROYED) ; drop (Thread :: from_raw (current)) ; } } }
}