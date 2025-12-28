macro_rules! deps {
    () => {
        Param!();
        LifetimeKind!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl LifetimeKind { fn is_elided (& self) -> bool { match self { LifetimeKind :: ImplicitObjectLifetimeDefault | LifetimeKind :: Infer => true , LifetimeKind :: Error | LifetimeKind :: Param (..) | LifetimeKind :: Static => false , } } }
    };
}

impl_111!();