macro_rules! deps {
    () => {
        StrippedStr!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < 's > StrippedStr < 's > { # [inline] fn new (data : & 's str) -> Self { Self { bytes : data . as_bytes () , state : State :: Ground , } } # [doc = " Create a [`String`] of the printable content"] # [inline] # [allow (clippy :: inherent_to_string_shadow_display)] pub fn to_string (& self) -> String { use std :: fmt :: Write as _ ; let mut stripped = String :: with_capacity (self . bytes . len ()) ; let _ = write ! (& mut stripped , "{self}") ; stripped } }
    };
}

impl_2!();