macro_rules! deps {
    () => {
        Transition!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Transition { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Transition(byte: {:X?}, next: {:?}, link: {:?})" , self . byte , self . next () . as_usize () , self . link () . as_usize ()) } }
    };
}

impl_88!()