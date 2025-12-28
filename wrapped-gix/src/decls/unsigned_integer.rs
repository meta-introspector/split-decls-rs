macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! unsigned_integer {
    () => {
        deps!();
        # [doc = ""] pub mod unsigned_integer { # [doc = " The error produced when failing to parse a signed integer from configuration."] pub type Error = super :: key :: Error < gix_config :: value :: Error , 'k' , 'u' > ; }
    };
}

unsigned_integer!();