macro_rules! PunycodeCodeUnit {
    () => {
        pub (crate) trait PunycodeCodeUnit { fn is_delimiter (& self) -> bool ; fn is_ascii (& self) -> bool ; fn digit (& self) -> Option < u32 > ; fn char (& self) -> char ; fn char_ascii_lower_case (& self) -> char ; }
    };
}

PunycodeCodeUnit!()