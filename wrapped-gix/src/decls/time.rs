macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! time {
    () => {
        deps!();
        # [doc = ""] pub mod time { # [doc = " The error produced when failing to parse time from configuration."] pub type Error = super :: key :: Error < gix_date :: parse :: Error , 't' , 'i' > ; }
    };
}

time!();