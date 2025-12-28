macro_rules! deps {
    () => {
        Url!();
        Connection!();
        Error!();
        AuthenticateFn!();
        Remote!();
        Note!();
    };
}

macro_rules! impl_937 {
    () => {
        deps!();
        # [doc = " Access"] impl < 'repo , T > Connection < '_ , 'repo , T > where T : Transport , { # [doc = " A utility to return a function that will use this repository's configuration to obtain credentials, similar to"] # [doc = " what `git credential` is doing."] # [doc = ""] # [doc = " It's meant to be used by users of the [`with_credentials()`](Self::with_credentials()) builder to gain access to the"] # [doc = " default way of handling credentials, which they can call as fallback."] pub fn configured_credentials (& self , url : gix_url :: Url ,) -> Result < AuthenticateFn < 'static > , crate :: config :: credential_helpers :: Error > { let (mut cascade , _action_with_normalized_url , prompt_opts) = self . remote . repo . config_snapshot () . credential_helpers (url) ? ; Ok (Box :: new (move | action | cascade . invoke (action , prompt_opts . clone ())) as AuthenticateFn < '_ >) } # [doc = " Return the underlying remote that instantiate this connection."] pub fn remote (& self) -> & Remote < 'repo > { self . remote } # [doc = " Provide a mutable transport to allow interacting with it according to its actual type."] # [doc = " Note that the caller _should not_ call [`configure()`](gix_protocol::transport::client::TransportWithoutIO::configure())"] # [doc = " as we will call it automatically before performing the handshake. Instead, to bring in custom configuration,"] # [doc = " call [`with_transport_options()`](Connection::with_transport_options())."] pub fn transport_mut (& mut self) -> & mut T { & mut self . transport . inner } }
    };
}

impl_937!()