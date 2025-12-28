macro_rules! deps {
    () => {
        Package!();
        DepKindInfo!();
        PackageId!();
    };
}

macro_rules! NodeDep {
    () => {
        deps!();
        # [derive (Clone , Serialize , Deserialize , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] # [doc = " A dependency in a node"] pub struct NodeDep { # [doc = " The name of the dependency's library target."] # [doc = " If the crate was renamed, it is the new name."] # [doc = ""] # [doc = " If -Zbindeps is enabled local references may result in an empty"] # [doc = " string."] # [doc = ""] # [doc = " After -Zbindeps gets stabilized, cargo has indicated this field"] # [doc = " will become deprecated."] pub name : String , # [doc = " Package ID (opaque unique identifier)"] pub pkg : PackageId , # [doc = " The kinds of dependencies."] # [doc = ""] # [doc = " This field was added in Rust 1.41."] # [serde (default)] pub dep_kinds : Vec < DepKindInfo > , }
    };
}

NodeDep!()