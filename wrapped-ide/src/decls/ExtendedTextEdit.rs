macro_rules! ExtendedTextEdit {
    () => {
        struct ExtendedTextEdit { edit : TextEdit , is_snippet : bool , }
    };
}

ExtendedTextEdit!();