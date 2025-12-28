macro_rules! deps {
    () => {
        CacheControl!();
    };
}

macro_rules! impl_1005 {
    () => {
        deps!();
        impl CacheControl { # [doc = " Get 'Cache-Control' header value."] # [must_use] pub fn value (& self) -> Option < String > { let mut value = if self . max_age > 0 { format ! ("max-age={}" , self . max_age) } else if self . max_age == - 1 { "no-cache" . to_string () } else { String :: new () } ; if ! self . public { if ! value . is_empty () { value += ", " ; } value += "private" ; } if ! value . is_empty () { Some (value) } else { None } } }
    };
}

impl_1005!()