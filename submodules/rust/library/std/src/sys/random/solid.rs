mkuse!{use crate :: sys :: pal :: abi ;}

macro_rules! fill_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_bytes in module {}", module_path!());
    };
}

mkfn!{
    fill_bytes_introspect!();
    pub fn fill_bytes (bytes : & mut [u8]) { unsafe { let result = abi :: SOLID_RNG_SampleRandomBytes (bytes . as_mut_ptr () , bytes . len ()) ; assert_eq ! (result , 0 , "failed to generate random data") ; } }
}