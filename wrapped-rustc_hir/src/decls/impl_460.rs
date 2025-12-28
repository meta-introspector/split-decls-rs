macro_rules! deps {
    () => {
        TraitItemId!();
        HashStableContext!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl < HirCtx : crate :: HashStableContext > ToStableHashKey < HirCtx > for TraitItemId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & HirCtx) -> DefPathHash { self . owner_id . def_id . to_stable_hash_key (hcx) } }
    };
}

impl_460!()