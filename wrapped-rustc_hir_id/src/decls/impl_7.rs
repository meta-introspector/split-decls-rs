macro_rules! deps {
    () => {
        OwnerId!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < CTX : HashStableContext > ToStableHashKey < CTX > for OwnerId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { hcx . def_path_hash (self . to_def_id ()) } }
    };
}

impl_7!();