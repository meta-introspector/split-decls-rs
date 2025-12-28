macro_rules! deps {
    () => {
        Init!();
    };
}

macro_rules! init {
    () => {
        deps!();
        # [doc = " Initialize all support necessary to access `dbghelp` API functions from this"] # [doc = " crate."] # [doc = ""] # [doc = " Note that this function is **safe**, it internally has its own"] # [doc = " synchronization. Also note that it is safe to call this function multiple"] # [doc = " times recursively."] pub fn init () -> Result < Init , () > { use core :: sync :: atomic :: { AtomicPtr , Ordering :: SeqCst } ; fn mutex_name () -> [u8 ; 33] { let mut name : [u8 ; 33] = * b"Local\\RustBacktraceMutex00000000\0" ; let mut id = unsafe { GetCurrentProcessId () } ; let mut index = name . len () - 1 ; while id > 0 { name [index - 1] = match (id & 0xF) as u8 { h @ 0 ..= 9 => b'0' + h , h => b'A' + (h - 10) , } ; id >>= 4 ; index -= 1 ; } name } unsafe { static LOCK : AtomicPtr < c_void > = AtomicPtr :: new (ptr :: null_mut ()) ; let mut lock = LOCK . load (SeqCst) ; if lock . is_null () { let name = mutex_name () ; lock = CreateMutexA (ptr :: null_mut () , FALSE , name . as_ptr ()) ; if lock . is_null () { return Err (()) ; } if let Err (other) = LOCK . compare_exchange (ptr :: null_mut () , lock , SeqCst , SeqCst) { debug_assert ! (! other . is_null ()) ; CloseHandle (lock) ; lock = other ; } } debug_assert ! (! lock . is_null ()) ; let r = WaitForSingleObjectEx (lock , INFINITE , FALSE) ; debug_assert_eq ! (r , 0) ; let ret = Init { lock } ; # [allow (static_mut_refs)] DBGHELP . ensure_open () ? ; static mut INITIALIZED : bool = false ; if ! INITIALIZED { set_optional_options (ret . dbghelp ()) ; INITIALIZED = true ; } Ok (ret) } }
    };
}

init!();