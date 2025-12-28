macro_rules! deps {
    () => {
        BodyId!();
        HashStableContext!();
    };
}

macro_rules! impl_458 {
    () => {
        deps!();
        impl < HirCtx : crate :: HashStableContext > ToStableHashKey < HirCtx > for BodyId { type KeyType = (DefPathHash , ItemLocalId) ; # [inline] fn to_stable_hash_key (& self , hcx : & HirCtx) -> (DefPathHash , ItemLocalId) { let BodyId { hir_id } = * self ; hir_id . to_stable_hash_key (hcx) } }
    };
}

impl_458!();