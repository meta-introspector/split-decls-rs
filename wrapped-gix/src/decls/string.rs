macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! string {
    () => {
        deps!();
        # [doc = ""] pub mod string { # [doc = " The error produced when failing to interpret configuration as UTF-8 encoded string."] pub type Error = super :: key :: Error < crate :: bstr :: Utf8Error , 'w' , 'd' > ; }
    };
}

string!()