macro_rules! deps {
    () => {
        ImplTraitId!();
    };
}

macro_rules! InternedOpaqueTyId {
    () => {
        deps!();
        # [salsa_macros :: interned (no_lifetime , debug , revisions = usize :: MAX)] # [derive (PartialOrd , Ord)] pub struct InternedOpaqueTyId { pub loc : ImplTraitId < 'db > , }
    };
}

InternedOpaqueTyId!();