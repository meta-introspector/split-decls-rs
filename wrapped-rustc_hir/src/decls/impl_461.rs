macro_rules! deps {
    () => {
        ImplItemId!();
        HashStableContext!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl < HirCtx : crate :: HashStableContext > ToStableHashKey < HirCtx > for ImplItemId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & HirCtx) -> DefPathHash { self . owner_id . def_id . to_stable_hash_key (hcx) } }
    };
}

impl_461!()