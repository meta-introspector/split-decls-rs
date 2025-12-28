macro_rules! impl_71 {
    () => {
        impl std :: fmt :: Debug for ClassLayout { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("ClassLayout") . field (& self . packing_size ()) . finish () } }
    };
}

impl_71!();