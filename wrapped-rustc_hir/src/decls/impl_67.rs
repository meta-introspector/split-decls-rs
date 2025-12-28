macro_rules! deps {
    () => {
        HashStableContext!();
        Namespace!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < CTX : crate :: HashStableContext > ToStableHashKey < CTX > for Namespace { type KeyType = Namespace ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> Namespace { * self } }
    };
}

impl_67!()