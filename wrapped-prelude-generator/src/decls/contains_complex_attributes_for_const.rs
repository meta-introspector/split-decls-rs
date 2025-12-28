macro_rules! contains_complex_attributes_for_const {
    () => {
        fn contains_complex_attributes_for_const (_constant : & ItemConst) -> bool { false }
    };
}

contains_complex_attributes_for_const!();