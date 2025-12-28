macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! lock_timeout {
    () => {
        deps!();
        # [doc = ""] pub mod lock_timeout { # [doc = " The error produced when failing to parse timeout for locks."] pub type Error = super :: key :: Error < gix_config :: value :: Error , 'i' , 'i' > ; }
    };
}

lock_timeout!();