macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! refspec {
    () => {
        deps!();
        # [doc = ""] pub mod refspec { # [doc = " The error produced when failing to parse a refspec from the configuration."] pub type Error = super :: key :: Error < gix_refspec :: parse :: Error , 'r' , 'p' > ; }
    };
}

refspec!();