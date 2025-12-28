macro_rules! deps {
    () => {
        FloatSIMDUtils!();
    };
}

macro_rules! FloatSIMDScalarUtils {
    () => {
        deps!();
        # [cfg (test)] pub (crate) trait FloatSIMDScalarUtils : FloatSIMDUtils { type Scalar ; fn replace (self , index : usize , new_value : Self :: Scalar) -> Self ; fn extract_lane (self , index : usize) -> Self :: Scalar ; }
    };
}

FloatSIMDScalarUtils!();