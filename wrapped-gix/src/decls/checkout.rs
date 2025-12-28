macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! checkout {
    () => {
        deps!();
        # [doc = ""] pub mod checkout { # [doc = ""] pub mod workers { use crate :: config ; # [doc = " The error produced when failing to parse the `checkout.workers` key."] pub type Error = config :: key :: Error < gix_config :: value :: Error , 'n' , 'd' > ; } }
    };
}

checkout!();