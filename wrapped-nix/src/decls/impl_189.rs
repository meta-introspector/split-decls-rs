macro_rules! deps {
    () => {
        TimeVal!();
        Result!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl fmt :: Display for TimeVal { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let (abs , sign) = if self . tv_sec () < 0 { (- * self , "-") } else { (* self , "") } ; let sec = abs . tv_sec () ; write ! (f , "{sign}") ? ; if abs . tv_usec () == 0 { if sec == 1 { write ! (f , "1 second") ? ; } else { write ! (f , "{sec} seconds") ? ; } } else if abs . tv_usec () % 1000 == 0 { write ! (f , "{sec}.{:03} seconds" , abs . tv_usec () / 1000) ? ; } else { write ! (f , "{sec}.{:06} seconds" , abs . tv_usec ()) ? ; } Ok (()) } }
    };
}

impl_189!()