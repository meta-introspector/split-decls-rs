macro_rules! BreakableKind {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) enum BreakableKind { Loop , For , While , Block , }
    };
}

BreakableKind!();