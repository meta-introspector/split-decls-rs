macro_rules! InternedLifetimeParamId {
    () => {
        # [salsa_macros :: interned (no_lifetime , debug , revisions = usize :: MAX)] # [derive (PartialOrd , Ord)] pub struct InternedLifetimeParamId { # [doc = " This stores the param and its index."] pub loc : (LifetimeParamId , u32) , }
    };
}

InternedLifetimeParamId!();