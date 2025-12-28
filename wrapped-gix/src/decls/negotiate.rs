macro_rules! deps {
    () => {
        Error!();
        Algorithm!();
    };
}

macro_rules! negotiate {
    () => {
        deps!();
        # [doc = ""] pub mod negotiate { # [cfg (feature = "credentials")] pub use gix_negotiate :: Algorithm ; # [cfg (any (feature = "blocking-network-client" , feature = "async-network-client"))] pub use gix_protocol :: fetch :: negotiate :: Error ; }
    };
}

negotiate!()