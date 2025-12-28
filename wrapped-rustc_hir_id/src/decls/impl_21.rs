macro_rules! deps {
    () => {
        HirId!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < CTX : rustc_span :: HashStableContext > ToStableHashKey < CTX > for HirId { type KeyType = (DefPathHash , ItemLocalId) ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> (DefPathHash , ItemLocalId) { let def_path_hash = self . owner . def_id . to_stable_hash_key (hcx) ; (def_path_hash , self . local_id) } }
    };
}

impl_21!()