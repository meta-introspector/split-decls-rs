macro_rules! deps {
    () => {
        MutblCap!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl MutblCap { # [must_use] fn cap_to_weakly_not (self , span : Option < Span >) -> Self { match self { MutblCap :: Not => MutblCap :: Not , _ => MutblCap :: WeaklyNot (span) , } } # [must_use] fn as_mutbl (self) -> Mutability { match self { MutblCap :: Not | MutblCap :: WeaklyNot (_) => Mutability :: Not , MutblCap :: Mut => Mutability :: Mut , } } }
    };
}

impl_322!()