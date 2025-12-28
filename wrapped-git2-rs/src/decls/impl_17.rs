macro_rules! impl_17 {
    () => {
        impl IndexEntryFlag { is_bit_set ! (is_extended , IndexEntryFlag :: EXTENDED) ; is_bit_set ! (is_valid , IndexEntryFlag :: VALID) ; }
    };
}

impl_17!();