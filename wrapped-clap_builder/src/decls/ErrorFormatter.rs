macro_rules! deps {
    () => {
        Error!();
        StyledStr!();
    };
}

macro_rules! ErrorFormatter {
    () => {
        deps!();
        # [doc = " Defines how to format an error for displaying to the user"] pub trait ErrorFormatter : Sized { # [doc = " Stylize the error for the terminal"] fn format_error (error : & crate :: error :: Error < Self >) -> StyledStr ; }
    };
}

ErrorFormatter!();