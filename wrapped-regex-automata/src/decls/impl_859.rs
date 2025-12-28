macro_rules! deps {
    () => {
        StateBuilderMatches!();
    };
}

macro_rules! impl_859 {
    () => {
        deps!();
        impl core :: fmt :: Debug for StateBuilderMatches { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_tuple ("StateBuilderMatches") . field (& self . repr ()) . finish () } }
    };
}

impl_859!();