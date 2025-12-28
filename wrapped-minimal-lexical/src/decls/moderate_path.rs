macro_rules! deps {
    () => {
        Number!();
        Float!();
        ExtendedFloat!();
    };
}

macro_rules! moderate_path {
    () => {
        deps!();
        # [doc = " Wrapper for different moderate-path algorithms."] # [doc = " A return exponent of `-1` indicates an invalid value."] # [inline] pub fn moderate_path < F : Float > (num : & Number) -> ExtendedFloat { # [cfg (not (feature = "compact"))] return lemire :: < F > (num) ; # [cfg (feature = "compact")] return bellerophon :: < F > (num) ; }
    };
}

moderate_path!();