macro_rules! DependencyKind {
    () => {
        # [derive (Eq , PartialEq , Clone , Debug , Copy , Hash , Serialize , Deserialize , Default)] # [doc = " Dependencies can come in three kinds"] pub enum DependencyKind { # [serde (rename = "normal")] # [default] # [doc = " The 'normal' kind"] Normal , # [serde (rename = "dev")] # [doc = " Those used in tests only"] Development , # [serde (rename = "build")] # [doc = " Those used in build scripts only"] Build , # [doc (hidden)] # [serde (other)] Unknown , }
    };
}

DependencyKind!();