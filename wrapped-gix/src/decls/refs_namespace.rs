macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! refs_namespace {
    () => {
        deps!();
        # [doc = ""] pub mod refs_namespace { # [doc = " The error produced when failing to parse a refspec from the configuration."] pub type Error = super :: key :: Error < gix_validate :: reference :: name :: Error , 'v' , 'i' > ; }
    };
}

refs_namespace!();