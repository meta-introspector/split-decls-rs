macro_rules! deps {
    () => {
        RegionErrors!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl std :: fmt :: Debug for RegionErrors < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_tuple ("RegionErrors") . field (& self . 0) . finish () } }
    };
}

impl_158!()