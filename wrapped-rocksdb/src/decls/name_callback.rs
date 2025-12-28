macro_rules! deps {
    () => {
        MergeFn!();
        MergeOperatorCallback!();
    };
}

macro_rules! name_callback {
    () => {
        deps!();
        pub unsafe extern "C" fn name_callback < F : MergeFn , PF : MergeFn > (raw_cb : * mut c_void ,) -> * const c_char { let cb = unsafe { & mut * (raw_cb as * mut MergeOperatorCallback < F , PF >) } ; cb . name . as_ptr () }
    };
}

name_callback!();