macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ssl_version {
    () => {
        deps!();
        # [doc = ""] pub mod ssl_version { # [doc = " The error produced when failing to parse a refspec from the configuration."] pub type Error = super :: key :: Error < std :: convert :: Infallible , 's' , 'i' > ; }
    };
}

ssl_version!();