macro_rules! StringOrBool {
    () => {
        # [derive (Clone , Debug , Serialize , Eq , PartialEq)] # [serde (untagged)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub enum StringOrBool { String (String) , Bool (bool) , }
    };
}

StringOrBool!()