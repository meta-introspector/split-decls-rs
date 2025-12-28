macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl Context { pub (crate) fn with_path < 'a > (& self , rela_path : & 'a BStr) -> driver :: apply :: Context < 'a , '_ > { driver :: apply :: Context { rela_path , ref_name : self . ref_name . as_ref () . map (AsRef :: as_ref) , treeish : self . treeish , blob : self . blob , } } }
    };
}

impl_114!()