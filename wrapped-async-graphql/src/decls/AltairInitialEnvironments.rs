macro_rules! deps {
    () => {
        AltairInitialEnvironmentState!();
    };
}

macro_rules! AltairInitialEnvironments {
    () => {
        deps!();
        # [doc = " Altair initial environments setup"] # [derive (Default , Serialize , Deserialize , JsonSchema)] # [serde (rename_all = "camelCase")] pub struct AltairInitialEnvironments { # [doc = " Base environment"] # [serde (default , skip_serializing_if = "Option::is_none")] pub base : Option < AltairInitialEnvironmentState > , # [doc = " Other sub environments"] # [serde (default , skip_serializing_if = "Vec::is_empty")] pub sub_environments : Vec < AltairInitialEnvironmentState > , }
    };
}

AltairInitialEnvironments!();