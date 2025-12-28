macro_rules! VecStringOrBool {
    () => {
        # [derive (PartialEq , Clone , Debug , Serialize)] # [serde (untagged)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum VecStringOrBool { VecString (Vec < String >) , Bool (bool) , }
    };
}

VecStringOrBool!()