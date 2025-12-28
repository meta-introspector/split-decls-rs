macro_rules! set_colors_ {
    () => {
        fn set_colors_ < S : AsHandle > (stream : & mut S , fg : anstyle :: AnsiColor , bg : anstyle :: AnsiColor ,) -> Result < () , inner :: IoError > { let handle = stream . as_handle () ; let handle = handle . as_raw_handle () ; let attributes = inner :: set_colors (fg , bg) ; inner :: set_console_text_attributes (handle , attributes) }
    };
}

set_colors_!()