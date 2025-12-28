macro_rules! deps {
    () => {
        Source!();
        DependencyKind!();
    };
}

macro_rules! Dependency {
    () => {
        deps!();
        # [derive (Clone , Serialize , Deserialize , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] # [doc = " A dependency of the main crate"] pub struct Dependency { # [doc = " Name as given in the `Cargo.toml`"] pub name : String , # [doc = " The source of dependency"] pub source : Option < Source > , # [doc = " The required version"] pub req : VersionReq , # [doc = " The kind of dependency this is"] # [serde (deserialize_with = "parse_dependency_kind")] pub kind : DependencyKind , # [doc = " Whether this dependency is required or optional"] pub optional : bool , # [doc = " Whether the default features in this dependency are used."] pub uses_default_features : bool , # [doc = " The list of features enabled for this dependency."] pub features : Vec < String > , # [doc = " The target this dependency is specific to."] # [doc = ""] # [doc = " Use the [`Display`] trait to access the contents."] # [doc = ""] # [doc = " [`Display`]: std::fmt::Display"] pub target : Option < Platform > , # [doc = " If the dependency is renamed, this is the new name for the dependency"] # [doc = " as a string.  None if it is not renamed."] pub rename : Option < String > , # [doc = " The URL of the index of the registry where this dependency is from."] # [doc = ""] # [doc = " If None, the dependency is from crates.io."] pub registry : Option < String > , # [doc = " The file system path for a local path dependency."] # [doc = ""] # [doc = " Only produced on cargo 1.51+"] pub path : Option < Utf8PathBuf > , }
    };
}

Dependency!()