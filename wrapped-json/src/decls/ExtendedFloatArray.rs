macro_rules! ExtendedFloatArray {
    () => {
        # [doc = " Precalculated powers that uses two-separate arrays for memory-efficiency."] # [doc (hidden)] pub (crate) struct ExtendedFloatArray { pub mant : & 'static [u64] , pub exp : & 'static [i32] , }
    };
}

ExtendedFloatArray!();