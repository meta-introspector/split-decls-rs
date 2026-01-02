mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: time :: Duration ;}
mkuse!{use crate :: { io , ptr } ;}
mkitem!{pub type Tid = hermit_abi :: Tid ;}
mkitem!{mkstruct!{pub struct Thread { tid : Tid , }}}
mkitem!{mkimpl!{unsafe impl Send for Thread { }}}
mkitem!{mkimpl!{unsafe impl Sync for Thread { }}}
mkitem!{pub const DEFAULT_MIN_STACK_SIZE : usize = 1 << 20 ;}
mkitem!{mkimpl!{impl Thread { pub unsafe fn new_with_coreid (stack : usize , p : Box < dyn FnOnce () > , core_id : isize ,) -> io :: Result < Thread > { let p = Box :: into_raw (Box :: new (p)) ; let tid = unsafe { hermit_abi :: spawn2 (thread_start , p . expose_provenance () , hermit_abi :: Priority :: into (hermit_abi :: NORMAL_PRIO) , stack , core_id ,) } ; return if tid == 0 { unsafe { drop (Box :: from_raw (p)) ; } Err (io :: const_error ! (io :: ErrorKind :: Uncategorized , "unable to create thread!")) } else { Ok (Thread { tid }) } ; extern "C" fn thread_start (main : usize) { unsafe { Box :: from_raw (ptr :: with_exposed_provenance :: < Box < dyn FnOnce () > > (main) . cast_mut ()) () ; crate :: sys :: thread_local :: destructors :: run () ; crate :: rt :: thread_cleanup () ; } } } pub unsafe fn new (stack : usize , _name : Option < & str > , p : Box < dyn FnOnce () > ,) -> io :: Result < Thread > { unsafe { Thread :: new_with_coreid (stack , p , - 1) } } pub fn join (self) { unsafe { let _ = hermit_abi :: join (self . tid) ; } } }}}

macro_rules! available_parallelism_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function available_parallelism in module {}", module_path!());
    };
}

mkfn!{
    available_parallelism_introspect!();
    pub fn available_parallelism () -> io :: Result < NonZero < usize > > { unsafe { Ok (NonZero :: new_unchecked (hermit_abi :: available_parallelism ())) } }
}

macro_rules! sleep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep in module {}", module_path!());
    };
}

mkfn!{
    sleep_introspect!();
    # [inline] pub fn sleep (dur : Duration) { let micros = dur . as_micros () + if dur . subsec_nanos () % 1_000 > 0 { 1 } else { 0 } ; let micros = u64 :: try_from (micros) . unwrap_or (u64 :: MAX) ; unsafe { hermit_abi :: usleep (micros) ; } }
}

macro_rules! yield_now_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function yield_now in module {}", module_path!());
    };
}

mkfn!{
    yield_now_introspect!();
    # [inline] pub fn yield_now () { unsafe { hermit_abi :: yield_now () ; } }
}