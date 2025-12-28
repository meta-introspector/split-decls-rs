macro_rules! deps {
    () => {
        MemberRef!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl std :: fmt :: Debug for MemberRef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("MemberRef") . field (& self . 0) . finish () } }
    };
}

impl_100!();