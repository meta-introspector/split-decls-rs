mkuse!{use crate :: sys :: c ;}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    # [cfg (not (target_vendor = "win7"))] # [inline] pub fn fill_bytes (bytes : & mut [u8]) { let ret = unsafe { c :: ProcessPrng (bytes . as_mut_ptr () , bytes . len ()) } ; debug_assert_eq ! (ret , c :: TRUE) ; }
}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    # [cfg (target_vendor = "win7")] pub fn fill_bytes (mut bytes : & mut [u8]) { while ! bytes . is_empty () { let len = bytes . len () . try_into () . unwrap_or (u32 :: MAX) ; let ret = unsafe { c :: RtlGenRandom (bytes . as_mut_ptr () . cast () , len) } ; assert ! (ret , "failed to generate random data") ; bytes = & mut bytes [len as usize ..] ; } }
}