macro_rules! FloatType {
    () => {
        # [doc = " All possible float type suffixes."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [non_exhaustive] pub enum FloatType { F32 , F64 , }
    };
}

FloatType!();