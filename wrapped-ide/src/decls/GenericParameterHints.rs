macro_rules! GenericParameterHints {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub struct GenericParameterHints { pub type_hints : bool , pub lifetime_hints : bool , pub const_hints : bool , }
    };
}

GenericParameterHints!()