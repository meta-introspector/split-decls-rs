macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_1027 {
    () => {
        deps!();
        # [doc = " Access"] impl Item { # [doc = " Return the relative path at which the item can currently be found in the working tree or index."] pub fn location (& self) -> & BStr { match self { Item :: IndexWorktree (change) => change . rela_path () , Item :: TreeIndex (change) => change . location () , } } }
    };
}

impl_1027!();