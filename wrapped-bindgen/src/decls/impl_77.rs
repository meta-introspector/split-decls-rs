macro_rules! impl_77 {
    () => {
        impl std :: fmt :: Debug for Field { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("Field") . field (& self . name ()) . finish () } }
    };
}

impl_77!();