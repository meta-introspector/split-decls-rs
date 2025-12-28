macro_rules! deps {
    () => {
        EdgeReferences!();
        IndexType!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < E , Ix : IndexType > Clone for EdgeReferences < '_ , E , Ix > { fn clone (& self) -> Self { EdgeReferences { iter : self . iter . clone () , } } }
    };
}

impl_309!()