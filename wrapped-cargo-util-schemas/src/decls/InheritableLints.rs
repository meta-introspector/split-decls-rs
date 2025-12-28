macro_rules! deps {
    () => {
        TomlLints!();
    };
}

macro_rules! InheritableLints {
    () => {
        deps!();
        # [derive (Serialize , Debug , Clone)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct InheritableLints { # [serde (skip_serializing_if = "std::ops::Not::not")] # [cfg_attr (feature = "unstable-schema" , schemars (default))] pub workspace : bool , # [serde (flatten)] pub lints : TomlLints , }
    };
}

InheritableLints!();