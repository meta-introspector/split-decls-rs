macro_rules! deps {
    () => {
        NodeOrToken!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < N : Deref , T : Deref > NodeOrToken < N , T > { pub (crate) fn as_deref (& self) -> NodeOrToken < & N :: Target , & T :: Target > { match self { NodeOrToken :: Node (node) => NodeOrToken :: Node (node) , NodeOrToken :: Token (token) => NodeOrToken :: Token (token) , } } }
    };
}

impl_102!()