macro_rules! InternedConstParamId {
    () => {
        # [salsa_macros :: interned (no_lifetime , debug , revisions = usize :: MAX)] # [derive (PartialOrd , Ord)] pub struct InternedConstParamId { pub loc : ConstParamId , }
    };
}

InternedConstParamId!()