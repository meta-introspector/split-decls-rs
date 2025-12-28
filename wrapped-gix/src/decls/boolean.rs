macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! boolean {
    () => {
        deps!();
        # [doc = ""] pub mod boolean { # [doc = " The error produced when failing to parse time from configuration."] pub type Error = super :: key :: Error < gix_config :: value :: Error , 'b' , 'i' > ; }
    };
}

boolean!()