macro_rules! deps {
    () => {
        LatticeOpKind!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl LatticeOpKind { fn invert (self) -> Self { match self { LatticeOpKind :: Glb => LatticeOpKind :: Lub , LatticeOpKind :: Lub => LatticeOpKind :: Glb , } } }
    };
}

impl_164!()