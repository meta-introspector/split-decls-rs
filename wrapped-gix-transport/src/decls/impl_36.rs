macro_rules! deps {
    () => {
        Error!();
        TransportWithoutIO!();
        SpawnProcessOnDemand!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl client :: TransportWithoutIO for SpawnProcessOnDemand { fn set_identity (& mut self , identity : gix_sec :: identity :: Account) -> Result < () , client :: Error > { if self . url . scheme == gix_url :: Scheme :: Ssh { self . url . set_user ((! identity . username . is_empty ()) . then_some (identity . username)) ; Ok (()) } else { Err (client :: Error :: AuthenticationUnsupported) } } fn to_url (& self) -> Cow < '_ , BStr > { Cow :: Owned (self . url . to_bstring ()) } fn connection_persists_across_multiple_requests (& self) -> bool { true } fn configure (& mut self , _config : & dyn Any) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { Ok (()) } }
    };
}

impl_36!()