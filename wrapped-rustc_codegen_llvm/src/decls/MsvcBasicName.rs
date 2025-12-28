macro_rules! MsvcBasicName {
    () => {
        trait MsvcBasicName { fn msvc_basic_name (self) -> & 'static str ; }
    };
}

MsvcBasicName!();