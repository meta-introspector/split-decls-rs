macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Sealed {
    () => {
        deps!();
        pub (crate) trait Sealed { fn symbol_name < R > (self , function : impl FnOnce (* const core :: ffi :: c_char) -> Result < R , crate :: Error > ,) -> Result < R , crate :: Error > ; }
    };
}

Sealed!()