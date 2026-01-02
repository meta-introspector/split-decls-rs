mkuse!{use crate :: fs :: File ;}
mkuse!{use crate :: io :: Read ;}
mkuse!{use crate :: sync :: OnceLock ;}
mkitem!{static SCHEME : OnceLock < File > = OnceLock :: new () ;}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { SCHEME . get_or_try_init (| | File :: open ("/scheme/rand")) . and_then (| mut scheme | scheme . read_exact (bytes)) . expect ("failed to generate random data") ; }
}