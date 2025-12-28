macro_rules! ListFormat {
    () => {
        # [derive (Debug , Default , Clone)] # [doc = " Enum representing the list format."] pub (crate) enum ListFormat { # [doc = " The regular, default format."] # [default] Pretty , # [doc = " The terse format, where nothing other than the name of the test and \": benchmark\" at the end"] # [doc = " is printed out."] Terse , }
    };
}

ListFormat!();