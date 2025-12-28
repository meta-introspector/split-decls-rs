macro_rules! deps {
    () => {
        Package!();
        Dependency!();
    };
}

macro_rules! Workspace {
    () => {
        deps!();
        # [derive (Debug , Default , Clone , PartialEq , Serialize , Deserialize)] pub struct Workspace { # [serde (default , skip_serializing_if = "Vec::is_empty")] pub members : Vec < String > , # [serde (skip_serializing_if = "Option::is_none")] pub resolver : Option < String > , # [serde (rename = "package" , skip_serializing_if = "Option::is_none")] pub package_config : Option < Package > , # [serde (rename = "dependencies" , default , skip_serializing_if = "HashMap::is_empty")] pub workspace_dependencies : HashMap < String , Dependency > , }
    };
}

Workspace!()