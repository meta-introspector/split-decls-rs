macro_rules! Visibility {
    () => {
        # [doc = " Must match the layout of `LLVMVisibility`."] # [repr (C)] # [derive (Copy , Clone , PartialEq , TryFromU32)] pub (crate) enum Visibility { Default = 0 , Hidden = 1 , Protected = 2 , }
    };
}

Visibility!();