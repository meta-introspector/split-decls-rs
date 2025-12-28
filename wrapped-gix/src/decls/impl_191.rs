macro_rules! deps {
    () => {
        Tree!();
        Error!();
        Editor!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        # [doc = " Tree editing"] # [cfg (feature = "tree-editor")] impl < 'repo > crate :: Tree < 'repo > { # [doc = " Start editing a new tree based on this one."] # [doc (alias = "treebuilder" , alias = "git2")] pub fn edit (& self) -> Result < super :: Editor < 'repo > , init :: Error > { super :: Editor :: new (self) } }
    };
}

impl_191!();