mkuse!{use super :: abi ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: io :: ErrorKind ;}
mkitem!{mkstruct!{# [doc = " Wraps a μITRON error code."] # [derive (Debug , Copy , Clone)] pub struct ItronError { er : abi :: ER , }}}
mkitem!{mkimpl!{impl ItronError { # [doc = " Constructs `ItronError` from the specified error code. Returns `None` if the"] # [doc = " error code does not represent a failure or warning."] # [inline] pub fn new (er : abi :: ER) -> Option < Self > { if er < 0 { Some (Self { er }) } else { None } } # [doc = " Returns `Ok(er)` if `er` represents a success or `Err(_)` otherwise."] # [inline] pub fn err_if_negative (er : abi :: ER) -> Result < abi :: ER , Self > { if let Some (error) = Self :: new (er) { Err (error) } else { Ok (er) } } # [doc = " Gets the raw error code."] # [inline] pub fn as_raw (& self) -> abi :: ER { self . er } }}}
mkitem!{mkimpl!{impl fmt :: Display for ItronError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (name) = crate :: sys :: error :: error_name (self . er) { write ! (f , "{} ({})" , name , self . er) } else { write ! (f , "{}" , self . er) } } }}}

macro_rules! error_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function error_name in module {}", module_path!());
    };
}

mkfn!{
    error_name_introspect!();
    # [doc = " Describe the specified μITRON error code. Returns `None` if it's an"] # [doc = " undefined error code."] pub fn error_name (er : abi :: ER) -> Option < & 'static str > { match er { er if er >= 0 => None , abi :: E_SYS => Some ("system error") , abi :: E_NOSPT => Some ("unsupported function") , abi :: E_RSFN => Some ("reserved function code") , abi :: E_RSATR => Some ("reserved attribute") , abi :: E_PAR => Some ("parameter error") , abi :: E_ID => Some ("invalid ID number") , abi :: E_CTX => Some ("context error") , abi :: E_MACV => Some ("memory access violation") , abi :: E_OACV => Some ("object access violation") , abi :: E_ILUSE => Some ("illegal service call use") , abi :: E_NOMEM => Some ("insufficient memory") , abi :: E_NOID => Some ("no ID number available") , abi :: E_OBJ => Some ("object state error") , abi :: E_NOEXS => Some ("non-existent object") , abi :: E_QOVR => Some ("queue overflow") , abi :: E_RLWAI => Some ("forced release from waiting") , abi :: E_TMOUT => Some ("polling failure or timeout") , abi :: E_DLT => Some ("waiting object deleted") , abi :: E_CLS => Some ("waiting object state changed") , abi :: E_WBLK => Some ("non-blocking code accepted") , abi :: E_BOVR => Some ("buffer overflow") , abi :: E_NORES => Some ("insufficient system resources") , abi :: E_RASTER => Some ("termination request raised") , abi :: E_COMM => Some ("communication failure") , _ => None , } }
}

macro_rules! is_interrupted_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_interrupted in module {}", module_path!());
    };
}

mkfn!{
    is_interrupted_introspect!();
    # [inline] pub fn is_interrupted (er : abi :: ER) -> bool { er == abi :: E_RLWAI }
}

macro_rules! decode_error_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_error_kind in module {}", module_path!());
    };
}

mkfn!{
    decode_error_kind_introspect!();
    pub fn decode_error_kind (er : abi :: ER) -> ErrorKind { match er { er if er >= 0 => ErrorKind :: Uncategorized , abi :: E_NOSPT => ErrorKind :: Unsupported , abi :: E_RSFN => ErrorKind :: InvalidInput , abi :: E_RSATR => ErrorKind :: InvalidInput , abi :: E_PAR => ErrorKind :: InvalidInput , abi :: E_ID => ErrorKind :: NotFound , abi :: E_MACV => ErrorKind :: PermissionDenied , abi :: E_OACV => ErrorKind :: PermissionDenied , abi :: E_NOMEM => ErrorKind :: OutOfMemory , abi :: E_NOID => ErrorKind :: OutOfMemory , abi :: E_NOEXS => ErrorKind :: NotFound , abi :: E_RLWAI => ErrorKind :: Interrupted , abi :: E_TMOUT => ErrorKind :: TimedOut , abi :: E_NORES => ErrorKind :: OutOfMemory , _ => ErrorKind :: Uncategorized , } }
}

macro_rules! expect_success_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expect_success in module {}", module_path!());
    };
}

mkfn!{
    expect_success_introspect!();
    # [doc = " Similar to `ItronError::err_if_negative(er).expect()` except that, while"] # [doc = " panicking, it prints the message to `panic_output` and aborts the program"] # [doc = " instead. This ensures the error message is not obscured by double"] # [doc = " panicking."] # [doc = ""] # [doc = " This is useful for diagnosing creation failures of synchronization"] # [doc = " primitives that are used by `std`'s internal mechanisms. Such failures"] # [doc = " are common when the system is mis-configured to provide a too-small pool for"] # [doc = " kernel objects."] # [inline] pub fn expect_success (er : abi :: ER , msg : & & str) -> abi :: ER { match ItronError :: err_if_negative (er) { Ok (x) => x , Err (e) => fail (e , msg) , } }
}

macro_rules! expect_success_aborting_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expect_success_aborting in module {}", module_path!());
    };
}

mkfn!{
    expect_success_aborting_introspect!();
    # [doc = " Similar to `ItronError::err_if_negative(er).expect()` but aborts instead."] # [doc = ""] # [doc = " Use this where panicking is not allowed or the effect of the failure"] # [doc = " would be persistent."] # [inline] pub fn expect_success_aborting (er : abi :: ER , msg : & & str) -> abi :: ER { match ItronError :: err_if_negative (er) { Ok (x) => x , Err (e) => fail_aborting (e , msg) , } }
}

macro_rules! fail_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fail in module {}", module_path!());
    };
}

mkfn!{
    fail_introspect!();
    # [cold] pub fn fail (e : impl fmt :: Display , msg : & & str) -> ! { if crate :: thread :: panicking () { fail_aborting (e , msg) } else { panic ! ("{} failed: {}" , * msg , e) } }
}

macro_rules! fail_aborting_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fail_aborting in module {}", module_path!());
    };
}

mkfn!{
    fail_aborting_introspect!();
    # [cold] pub fn fail_aborting (e : impl fmt :: Display , msg : & & str) -> ! { rtabort ! ("{} failed: {}" , * msg , e) }
}