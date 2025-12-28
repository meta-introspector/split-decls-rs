macro_rules! deps {
    () => {
        VisibilityKind!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl VisibilityKind { pub fn is_pub (& self) -> bool { matches ! (self , VisibilityKind :: Public) } }
    };
}

impl_202!()