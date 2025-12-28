macro_rules! impl_22 {
    () => {
        impl < CTX : HashStableContext > ToStableHashKey < CTX > for ItemLocalId { type KeyType = ItemLocalId ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> ItemLocalId { * self } }
    };
}

impl_22!()