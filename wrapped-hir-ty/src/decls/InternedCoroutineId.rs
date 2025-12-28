macro_rules! deps {
    () => {
        InternedCoroutine!();
    };
}

macro_rules! InternedCoroutineId {
    () => {
        deps!();
        # [salsa_macros :: interned (no_lifetime , debug , revisions = usize :: MAX)] # [derive (PartialOrd , Ord)] pub struct InternedCoroutineId { pub loc : InternedCoroutine , }
    };
}

InternedCoroutineId!();