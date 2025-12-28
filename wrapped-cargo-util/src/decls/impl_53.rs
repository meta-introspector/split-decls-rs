macro_rules! deps {
    () => {
        ProcessBuilder!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl fmt :: Display for ProcessBuilder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "`") ? ; if self . display_env_vars { for (key , val) in self . env . iter () { if let Some (val) = val { let val = escape (val . to_string_lossy ()) ; if cfg ! (windows) { write ! (f , "set {}={}&& " , key , val) ? ; } else { write ! (f , "{}={} " , key , val) ? ; } } } } write ! (f , "{}" , self . get_program () . to_string_lossy ()) ? ; for arg in self . get_args () { write ! (f , " {}" , escape (arg . to_string_lossy ())) ? ; } write ! (f , "`") } }
    };
}

impl_53!()