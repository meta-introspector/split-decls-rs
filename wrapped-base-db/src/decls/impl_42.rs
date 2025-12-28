macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl fmt :: Debug for Env { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct EnvDebug < 's > (Vec < (& 's String , & 's String) >) ; impl fmt :: Debug for EnvDebug < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . 0 . iter () . copied ()) . finish () } } f . debug_struct ("Env") . field ("entries" , & { let mut entries : Vec < _ > = self . entries . iter () . collect () ; entries . sort () ; EnvDebug (entries) }) . finish () } }
    };
}

impl_42!()