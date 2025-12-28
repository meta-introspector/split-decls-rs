macro_rules! deps {
    () => {
        InternedClosure!();
    };
}

macro_rules! InternedClosureId {
    () => {
        deps!();
        # [salsa_macros :: interned (no_lifetime , debug , revisions = usize :: MAX)] # [derive (PartialOrd , Ord)] pub struct InternedClosureId { pub loc : InternedClosure , }
    };
}

InternedClosureId!();