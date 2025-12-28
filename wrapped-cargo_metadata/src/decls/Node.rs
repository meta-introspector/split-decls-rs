macro_rules! deps {
    () => {
        PackageId!();
        NodeDep!();
    };
}

macro_rules! Node {
    () => {
        deps!();
        # [derive (Clone , Serialize , Deserialize , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] # [doc = " A node in a dependencies graph"] pub struct Node { # [doc = " An opaque identifier for a package"] pub id : PackageId , # [doc = " Dependencies in a structured format."] # [doc = ""] # [doc = " `deps` handles renamed dependencies whereas `dependencies` does not."] # [serde (default)] pub deps : Vec < NodeDep > , # [doc = " List of opaque identifiers for this node's dependencies."] # [doc = " It doesn't support renamed dependencies. See `deps`."] pub dependencies : Vec < PackageId > , # [doc = " Features enabled on the crate"] # [serde (default)] pub features : Vec < FeatureName > , }
    };
}

Node!()