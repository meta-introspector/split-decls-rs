macro_rules! deps {
    () => {
        IdxRange!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T > fmt :: Debug for IdxRange < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple (& format ! ("IdxRange::<{}>" , std :: any :: type_name ::< T > ())) . field (& self . range) . finish () } }
    };
}

impl_42!()