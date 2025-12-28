macro_rules! deps {
    () => {
        Display!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl fmt :: Display for Display { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let bytes = self . byte_size . as_u64 () ; let unit = self . format . unit () ; # [allow (unused_variables)] let unit_base = self . format . unit_base () ; let unit_prefixes = self . format . unit_prefixes () ; let unit_separator = self . format . unit_separator () ; let unit_suffix = self . format . unit_suffix () ; let precision = f . precision () . unwrap_or (1) ; if bytes < unit { write ! (f , "{bytes}{unit_separator}B") ? ; } else { let size = bytes as f64 ; # [cfg (feature = "std")] let exp = ideal_unit_std (size , unit_base) ; # [cfg (not (feature = "std"))] let exp = ideal_unit_no_std (size , unit) ; let unit_prefix = unit_prefixes [exp - 1] as char ; write ! (f , "{:.precision$}{unit_separator}{unit_prefix}{unit_suffix}" , (size / unit . pow (exp as u32) as f64) ,) ? ; } Ok (()) } }
    };
}

impl_7!();