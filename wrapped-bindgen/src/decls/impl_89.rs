macro_rules! impl_89 {
    () => {
        impl std :: fmt :: Debug for MemberRef { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("MemberRef") . field (& self . 0) . finish () } }
    };
}

impl_89!()