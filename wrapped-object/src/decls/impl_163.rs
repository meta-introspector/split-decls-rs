macro_rules! deps {
    () => {
        ReadRef!();
        Symbol!();
        Result!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > fmt :: Debug for Symbol < 'data , 'file , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Symbol") . field ("name" , & self . name () . unwrap_or ("<invalid>")) . field ("address" , & self . address ()) . field ("size" , & self . size ()) . field ("kind" , & self . kind ()) . field ("section" , & self . section ()) . field ("scope" , & self . scope ()) . field ("weak" , & self . is_weak ()) . field ("flags" , & self . flags ()) . finish () } }
    };
}

impl_163!();