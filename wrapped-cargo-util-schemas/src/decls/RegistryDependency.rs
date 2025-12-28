macro_rules! deps {
    () => {
        IndexPackage!();
    };
}

macro_rules! RegistryDependency {
    () => {
        deps!();
        # [doc = " A dependency as encoded in the [`IndexPackage`] index JSON."] # [derive (Deserialize , Serialize , Clone)] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct RegistryDependency < 'a > { # [doc = " Name of the dependency. If the dependency is renamed, the original"] # [doc = " would be stored in [`RegistryDependency::package`]."] # [serde (borrow)] pub name : Cow < 'a , str > , # [doc = " The SemVer requirement for this dependency."] # [serde (borrow)] pub req : Cow < 'a , str > , # [doc = " Set of features enabled for this dependency."] # [serde (default)] pub features : Vec < Cow < 'a , str > > , # [doc = " Whether or not this is an optional dependency."] # [serde (default)] pub optional : bool , # [doc = " Whether or not default features are enabled."] # [serde (default = "default_true")] pub default_features : bool , # [doc = " The target platform for this dependency."] pub target : Option < Cow < 'a , str > > , # [doc = " The dependency kind. \"dev\", \"build\", and \"normal\"."] pub kind : Option < Cow < 'a , str > > , # [doc = " The URL of the index of the registry where this dependency is from."] # [doc = " `None` if it is from the same index."] pub registry : Option < Cow < 'a , str > > , # [doc = " The original name if the dependency is renamed."] pub package : Option < Cow < 'a , str > > , # [doc = " Whether or not this is a public dependency. Unstable. See [RFC 1977]."] # [doc = ""] # [doc = " [RFC 1977]: https://rust-lang.github.io/rfcs/1977-public-private-dependencies.html"] pub public : Option < bool > , # [doc = " The artifacts to build from this dependency."] pub artifact : Option < Vec < Cow < 'a , str > > > , # [doc = " The target for bindep."] pub bindep_target : Option < Cow < 'a , str > > , # [doc = " Whether or not this is a library dependency."] # [serde (default)] pub lib : bool , }
    };
}

RegistryDependency!();