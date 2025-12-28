macro_rules! deps {
    () => {
        UseTree!();
        UseTreeKind!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl UseTree { pub fn ident (& self) -> Ident { match self . kind { UseTreeKind :: Simple (Some (rename)) => rename , UseTreeKind :: Simple (None) => { self . prefix . segments . last () . expect ("empty prefix in a simple import") . ident } _ => panic ! ("`UseTree::ident` can only be used on a simple import") , } } }
    };
}

impl_187!()