macro_rules! deps {
    () => {
        Result!();
        Segment!();
        ReadRef!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > fmt :: Debug for Segment < 'data , 'file , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("Segment") ; match self . name () { Ok (Some (ref name)) => { s . field ("name" , name) ; } Ok (None) => { } Err (_) => { s . field ("name" , & "<invalid>") ; } } s . field ("address" , & self . address ()) . field ("size" , & self . size ()) . finish () } }
    };
}

impl_132!();