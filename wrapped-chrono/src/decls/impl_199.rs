macro_rules! deps {
    () => {
        Item!();
        DelayedFormat!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'a , I : Iterator < Item = B > + Clone , B : Borrow < Item < 'a > > > Display for DelayedFormat < I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut result = String :: new () ; self . write_to (& mut result) ? ; f . pad (& result) } }
    };
}

impl_199!()