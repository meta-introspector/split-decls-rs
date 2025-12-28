macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < S > From < S > for Cache < S > where S : gix_pack :: Find , { fn from (store : S) -> Self { Self { inner : store , pack_cache : None , new_pack_cache : None , object_cache : None , new_object_cache : None , } } }
    };
}

impl_139!()