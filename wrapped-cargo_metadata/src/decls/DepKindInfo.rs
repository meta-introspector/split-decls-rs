macro_rules! deps {
    () => {
        DependencyKind!();
    };
}

macro_rules! DepKindInfo {
    () => {
        deps!();
        # [derive (Clone , Serialize , Deserialize , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] # [doc = " Information about a dependency kind."] pub struct DepKindInfo { # [doc = " The kind of dependency."] # [serde (deserialize_with = "dependency::parse_dependency_kind")] pub kind : DependencyKind , # [doc = " The target platform for the dependency."] # [doc = ""] # [doc = " This is `None` if it is not a target dependency."] # [doc = ""] # [doc = " Use the [`Display`] trait to access the contents."] # [doc = ""] # [doc = " By default all platform dependencies are included in the resolve"] # [doc = " graph. Use Cargo's `--filter-platform` flag if you only want to"] # [doc = " include dependencies for a specific platform."] # [doc = ""] # [doc = " [`Display`]: std::fmt::Display"] pub target : Option < dependency :: Platform > , }
    };
}

DepKindInfo!();