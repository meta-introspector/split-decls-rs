macro_rules! deps {
    () => {
        Regex!();
    };
}

macro_rules! DFAEngine {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct DFAEngine (# [cfg (feature = "dfa-build")] dfa :: regex :: Regex , # [cfg (not (feature = "dfa-build"))] () ,) ;
    };
}

DFAEngine!()