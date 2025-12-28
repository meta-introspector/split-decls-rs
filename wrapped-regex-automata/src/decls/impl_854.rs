macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_854 {
    () => {
        deps!();
        impl core :: fmt :: Debug for State { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_tuple ("State") . field (& self . repr ()) . finish () } }
    };
}

impl_854!();