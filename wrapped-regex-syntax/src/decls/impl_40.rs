macro_rules! deps {
    () => {
        Ast!();
        Concat!();
        Group!();
        Frame!();
        Repetition!();
        Alternation!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'a > Frame < 'a > { # [doc = " Perform the next inductive step on this frame and return the next"] # [doc = " child AST node to visit."] fn child (& self) -> & 'a Ast { match * self { Frame :: Repetition (rep) => & rep . ast , Frame :: Group (group) => & group . ast , Frame :: Concat { head , .. } => head , Frame :: Alternation { head , .. } => head , } } }
    };
}

impl_40!()