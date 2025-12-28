macro_rules! deps {
    () => {
        ExtendedTextEdit!();
    };
}

macro_rules! conv {
    () => {
        deps!();
        fn conv (edit : TextEdit) -> ExtendedTextEdit { ExtendedTextEdit { edit , is_snippet : false } }
    };
}

conv!()