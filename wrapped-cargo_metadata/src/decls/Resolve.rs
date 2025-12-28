macro_rules! deps {
    () => {
        PackageId!();
        Node!();
    };
}

macro_rules! Resolve {
    () => {
        deps!();
        # [derive (Clone , Serialize , Deserialize , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] # [doc = " A dependency graph"] pub struct Resolve { # [doc = " Nodes in a dependencies graph"] pub nodes : Vec < Node > , # [doc = " The crate for which the metadata was read."] pub root : Option < PackageId > , }
    };
}

Resolve!();