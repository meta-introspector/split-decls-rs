macro_rules! deps {
    () => {
        Upgraded!();
        OnUpgrade!();
        Error!();
        Result!();
        UpgradeExpected!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl Future for OnUpgrade { type Output = Result < Upgraded , crate :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . rx { Some (ref rx) => Pin :: new (& mut * rx . lock () . unwrap ()) . poll (cx) . map (| res | match res { Ok (Ok (upgraded)) => Ok (upgraded) , Ok (Err (err)) => Err (err) , Err (_oneshot_canceled) => { Err (crate :: Error :: new_canceled () . with (UpgradeExpected)) } }) , None => Poll :: Ready (Err (crate :: Error :: new_user_no_upgrade ())) , } } }
    };
}

impl_219!()