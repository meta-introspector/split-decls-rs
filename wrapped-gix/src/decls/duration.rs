macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! duration {
    () => {
        deps!();
        # [doc = ""] pub mod duration { # [doc = " The error produced when failing to parse durations (in milliseconds)."] pub type Error = super :: key :: Error < gix_config :: value :: Error , 'd' , 'i' > ; }
    };
}

duration!()