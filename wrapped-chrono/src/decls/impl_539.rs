macro_rules! deps {
    () => {
        FixedOffset!();
    };
}

macro_rules! impl_539 {
    () => {
        deps!();
        impl fmt :: Debug for FixedOffset { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let offset = self . local_minus_utc ; let (sign , offset) = if offset < 0 { ('-' , - offset) } else { ('+' , offset) } ; let sec = offset . rem_euclid (60) ; let mins = offset . div_euclid (60) ; let min = mins . rem_euclid (60) ; let hour = mins . div_euclid (60) ; if sec == 0 { write ! (f , "{sign}{hour:02}:{min:02}") } else { write ! (f , "{sign}{hour:02}:{min:02}:{sec:02}") } } }
    };
}

impl_539!();