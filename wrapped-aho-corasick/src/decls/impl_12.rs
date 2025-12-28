macro_rules! deps {
    () => {
        AhoCorasick!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl core :: fmt :: Debug for AhoCorasick { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_tuple ("AhoCorasick") . field (& self . aut) . finish () } }
    };
}

impl_12!()