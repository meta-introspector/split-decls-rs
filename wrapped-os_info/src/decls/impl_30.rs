macro_rules! deps {
    () => {
        Info!();
        Version!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Display for Info { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { write ! (f , "{}" , self . os_type) ? ; if self . version != Version :: Unknown { write ! (f , " {}" , self . version) ? ; } if let Some (ref edition) = self . edition { write ! (f , " ({edition})") ? ; } if let Some (ref codename) = self . codename { write ! (f , " ({codename})") ? ; } write ! (f , " [{}]" , self . bitness) } }
    };
}

impl_30!();