macro_rules! deps {
    () => {
        ExtendedFloat!();
        Float!();
    };
}

macro_rules! round_to_native {
    () => {
        deps!();
        # [inline] pub (crate) fn round_to_native < F , Algorithm > (fp : & mut ExtendedFloat , algorithm : Algorithm) where F : Float , Algorithm : FnOnce (& mut ExtendedFloat , i32) , { fp . normalize () ; round_to_float :: < F , _ > (fp , algorithm) ; avoid_overflow :: < F > (fp) ; }
    };
}

round_to_native!()