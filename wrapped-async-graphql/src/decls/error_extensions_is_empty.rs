macro_rules! deps {
    () => {
        ErrorExtensionValues!();
    };
}

macro_rules! error_extensions_is_empty {
    () => {
        deps!();
        fn error_extensions_is_empty (values : & Option < ErrorExtensionValues >) -> bool { values . as_ref () . is_none_or (| values | values . 0 . is_empty ()) }
    };
}

error_extensions_is_empty!();