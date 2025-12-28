macro_rules! deps {
    () => {
        Frame!();
        Capture!();
        Alternation!();
        Hir!();
        Repetition!();
        Concat!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < 'a > Frame < 'a > { # [doc = " Perform the next inductive step on this frame and return the next"] # [doc = " child HIR node to visit."] fn child (& self) -> & 'a Hir { match * self { Frame :: Repetition (rep) => & rep . sub , Frame :: Capture (capture) => & capture . sub , Frame :: Concat { head , .. } => head , Frame :: Alternation { head , .. } => head , } } }
    };
}

impl_210!()