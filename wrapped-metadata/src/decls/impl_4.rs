macro_rules! impl_4 {
    () => {
        impl FieldAttributes { pub const Private : Self = Self (0x1) ; pub const Public : Self = Self (0x6) ; pub const Literal : Self = Self (0x40) ; pub const Static : Self = Self (0x10) ; pub const SpecialName : Self = Self (0x200) ; pub const RTSpecialName : Self = Self (0x400) ; pub const HasDefault : Self = Self (0x8000) ; }
    };
}

impl_4!()