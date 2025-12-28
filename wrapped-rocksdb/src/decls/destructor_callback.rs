macro_rules! deps {
    () => {
        MergeOperatorCallback!();
        MergeFn!();
    };
}

macro_rules! destructor_callback {
    () => {
        deps!();
        pub unsafe extern "C" fn destructor_callback < F : MergeFn , PF : MergeFn > (raw_cb : * mut c_void) { drop (unsafe { Box :: from_raw (raw_cb as * mut MergeOperatorCallback < F , PF >) }) ; }
    };
}

destructor_callback!()