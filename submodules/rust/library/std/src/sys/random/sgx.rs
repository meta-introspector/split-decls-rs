mkuse!{use crate :: arch :: x86_64 :: { _rdrand16_step , _rdrand32_step , _rdrand64_step } ;}
mkitem!{const RETRIES : u32 = 10 ;}

macro_rules! fail_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fail in module {}", module_path!());
    };
}

mkfn!{
    fail_introspect!();
    fn fail () -> ! { panic ! ("failed to generate random data") ; }
}

macro_rules! rdrand64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rdrand64 in module {}", module_path!());
    };
}

mkfn!{
    rdrand64_introspect!();
    fn rdrand64 () -> u64 { unsafe { let mut ret : u64 = 0 ; for _ in 0 .. RETRIES { if _rdrand64_step (& mut ret) == 1 { return ret ; } } fail () ; } }
}

macro_rules! rdrand32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rdrand32 in module {}", module_path!());
    };
}

mkfn!{
    rdrand32_introspect!();
    fn rdrand32 () -> u32 { unsafe { let mut ret : u32 = 0 ; for _ in 0 .. RETRIES { if _rdrand32_step (& mut ret) == 1 { return ret ; } } fail () ; } }
}

macro_rules! rdrand16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rdrand16 in module {}", module_path!());
    };
}

mkfn!{
    rdrand16_introspect!();
    fn rdrand16 () -> u16 { unsafe { let mut ret : u16 = 0 ; for _ in 0 .. RETRIES { if _rdrand16_step (& mut ret) == 1 { return ret ; } } fail () ; } }
}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { let (chunks , remainder) = bytes . as_chunks_mut () ; for chunk in chunks { * chunk = rdrand64 () . to_ne_bytes () ; } let (chunks , remainder) = remainder . as_chunks_mut () ; for chunk in chunks { * chunk = rdrand32 () . to_ne_bytes () ; } let (chunks , remainder) = remainder . as_chunks_mut () ; for chunk in chunks { * chunk = rdrand16 () . to_ne_bytes () ; } if let [byte] = remainder { * byte = rdrand16 () as u8 ; } }
}