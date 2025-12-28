macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < S : Clone > Clone for Cache < S > { fn clone (& self) -> Self { Cache { inner : self . inner . clone () , new_pack_cache : self . new_pack_cache . clone () , new_object_cache : self . new_object_cache . clone () , pack_cache : self . new_pack_cache . as_ref () . map (| create | RefCell :: new (create ())) , object_cache : self . new_object_cache . as_ref () . map (| create | RefCell :: new (create ())) , } } }
    };
}

impl_140!()