macro_rules! AutoCrlf {
    () => {
        # [doc = " Possible states for the `core.autocrlf`."] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq)] pub enum AutoCrlf { # [doc = " The same as if the `text eol=lf` attribute is set."] Input , # [doc = " The same as if the `text eol=crlf` attribute is set."] Enabled , # [doc = " No conversion is performed."] # [default] Disabled , }
    };
}

AutoCrlf!()