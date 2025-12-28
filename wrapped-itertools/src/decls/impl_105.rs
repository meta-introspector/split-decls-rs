macro_rules! deps {
    () => {
        Tuple1Combination!();
        HasCombination!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < I : Iterator > HasCombination < I > for (I :: Item ,) { type Combination = Tuple1Combination < I > ; }
    };
}

impl_105!();