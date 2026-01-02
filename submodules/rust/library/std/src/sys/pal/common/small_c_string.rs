mkuse!{use crate :: ffi :: { CStr , CString } ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: path :: Path ;}
mkuse!{use crate :: { io , ptr , slice } ;}
mkitem!{# [cfg (not (target_os = "espidf"))] const MAX_STACK_ALLOCATION : usize = 384 ;}
mkitem!{# [cfg (target_os = "espidf")] const MAX_STACK_ALLOCATION : usize = 32 ;}
mkitem!{const NUL_ERR : io :: Error = io :: const_error ! (io :: ErrorKind :: InvalidInput , "file name contained an unexpected NUL byte") ;}

macro_rules! run_path_with_cstr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_path_with_cstr in module {}", module_path!());
    };
}

mkfn!{
    run_path_with_cstr_introspect!();
    # [inline] pub fn run_path_with_cstr < T > (path : & Path , f : & dyn Fn (& CStr) -> io :: Result < T >) -> io :: Result < T > { run_with_cstr (path . as_os_str () . as_encoded_bytes () , f) }
}

macro_rules! run_with_cstr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_with_cstr in module {}", module_path!());
    };
}

mkfn!{
    run_with_cstr_introspect!();
    # [inline] pub fn run_with_cstr < T > (bytes : & [u8] , f : & dyn Fn (& CStr) -> io :: Result < T >) -> io :: Result < T > { if bytes . len () >= MAX_STACK_ALLOCATION { run_with_cstr_allocating (bytes , f) } else { unsafe { run_with_cstr_stack (bytes , f) } } }
}

macro_rules! run_with_cstr_stack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_with_cstr_stack in module {}", module_path!());
    };
}

mkfn!{
    run_with_cstr_stack_introspect!();
    # [doc = " # Safety"] # [doc = ""] # [doc = " `bytes` must have a length less than `MAX_STACK_ALLOCATION`."] unsafe fn run_with_cstr_stack < T > (bytes : & [u8] , f : & dyn Fn (& CStr) -> io :: Result < T > ,) -> io :: Result < T > { let mut buf = MaybeUninit :: < [u8 ; MAX_STACK_ALLOCATION] > :: uninit () ; let buf_ptr = buf . as_mut_ptr () as * mut u8 ; unsafe { ptr :: copy_nonoverlapping (bytes . as_ptr () , buf_ptr , bytes . len ()) ; buf_ptr . add (bytes . len ()) . write (0) ; } match CStr :: from_bytes_with_nul (unsafe { slice :: from_raw_parts (buf_ptr , bytes . len () + 1) }) { Ok (s) => f (s) , Err (_) => Err (NUL_ERR) , } }
}

macro_rules! run_with_cstr_allocating_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_with_cstr_allocating in module {}", module_path!());
    };
}

mkfn!{
    run_with_cstr_allocating_introspect!();
    # [cold] # [inline (never)] fn run_with_cstr_allocating < T > (bytes : & [u8] , f : & dyn Fn (& CStr) -> io :: Result < T >) -> io :: Result < T > { match CString :: new (bytes) { Ok (s) => f (& s) , Err (_) => Err (NUL_ERR) , } }
}