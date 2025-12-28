macro_rules! deps {
    () => {
        SerializableString!();
        StringComponent!();
    };
}

macro_rules! impl_serializable_string_for_fixed_size {
    () => {
        deps!();
        macro_rules ! impl_serializable_string_for_fixed_size { ($ n : expr) => { impl <'a > SerializableString for [StringComponent <'a >; $ n] { # [inline (always)] fn serialized_size (& self) -> usize { (& self [..]) . serialized_size () } # [inline (always)] fn serialize (& self , bytes : & mut [u8]) { (& self [..]) . serialize (bytes) ; } } } ; }
    };
}

impl_serializable_string_for_fixed_size!()