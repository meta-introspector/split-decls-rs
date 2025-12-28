macro_rules! deps {
    () => {
        StateBuilderNFA!();
    };
}

macro_rules! impl_862 {
    () => {
        deps!();
        impl core :: fmt :: Debug for StateBuilderNFA { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_tuple ("StateBuilderNFA") . field (& self . repr ()) . finish () } }
    };
}

impl_862!();