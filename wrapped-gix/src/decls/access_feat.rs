macro_rules! deps {
    () => {
        Error!();
        Connection!();
        PrepareFetch!();
        Options!();
    };
}

macro_rules! access_feat {
    () => {
        deps!();
        # [cfg (any (feature = "async-network-client" , feature = "blocking-network-client"))] mod access_feat { use super :: Transport ; use crate :: clone :: PrepareFetch ; # [doc = " Builder"] impl PrepareFetch { # [doc = " Set a callback to use for configuring the connection to use right before connecting to the remote."] # [doc = ""] # [doc = " It is most commonly used for custom configuration."] pub fn configure_connection (mut self , f : impl FnMut (& mut crate :: remote :: Connection < '_ , '_ , Box < dyn Transport + Send > > ,) -> Result < () , Box < dyn std :: error :: Error + Send + Sync > > + 'static ,) -> Self { self . configure_connection = Some (Box :: new (f)) ; self } # [doc = " Set additional options to adjust parts of the fetch operation that are not affected by the git configuration."] pub fn with_fetch_options (mut self , opts : crate :: remote :: ref_map :: Options) -> Self { self . fetch_options = opts ; self } } }
    };
}

access_feat!()