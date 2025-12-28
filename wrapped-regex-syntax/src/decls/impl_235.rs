macro_rules! deps {
    () => {
        Result!();
        Formatter!();
        ClassUnicodeRange!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl core :: fmt :: Debug for ClassUnicodeRange { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let start = if ! self . start . is_whitespace () && ! self . start . is_control () { self . start . to_string () } else { format ! ("0x{:X}" , u32 :: from (self . start)) } ; let end = if ! self . end . is_whitespace () && ! self . end . is_control () { self . end . to_string () } else { format ! ("0x{:X}" , u32 :: from (self . end)) } ; f . debug_struct ("ClassUnicodeRange") . field ("start" , & start) . field ("end" , & end) . finish () } }
    };
}

impl_235!();