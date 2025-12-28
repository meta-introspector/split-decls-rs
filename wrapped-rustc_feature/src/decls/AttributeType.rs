macro_rules! AttributeType {
    () => {
        # [derive (Copy , Clone , PartialEq , Debug)] pub enum AttributeType { # [doc = " Normal, builtin attribute that is consumed"] # [doc = " by the compiler before the unused_attribute check"] Normal , # [doc = " Builtin attribute that is only allowed at the crate level"] CrateLevel , }
    };
}

AttributeType!()