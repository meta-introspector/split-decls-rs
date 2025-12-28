macro_rules! AltairInitialEnvironmentState {
    () => {
        # [doc = " Altair initial environment state"] # [derive (Default , Serialize , Deserialize , JsonSchema)] # [serde (rename_all = "camelCase")] pub struct AltairInitialEnvironmentState { # [doc = " Environment identifier"] # [serde (default , skip_serializing_if = "Option::is_none")] pub id : Option < String > , # [doc = " Environment title"] # [serde (default , skip_serializing_if = "Option::is_none")] pub title : Option < String > , # [doc = " Environment variables"] # [serde (default , skip_serializing_if = "HashMap::is_empty")] pub variables : HashMap < String , String > , }
    };
}

AltairInitialEnvironmentState!()