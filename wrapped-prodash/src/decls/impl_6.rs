macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Debug for Item { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Item") . field ("key" , & self . key) . field ("value" , & self . value) . finish_non_exhaustive () } }
    };
}

impl_6!();