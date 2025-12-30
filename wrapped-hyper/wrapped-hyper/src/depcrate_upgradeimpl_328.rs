// Generated macro for impl_328 (impl)
macro_rules! Depcrate_upgradeimpl_328 {
() => {
// Module: crate::upgrade
// Provides: {"impl_328"}
// Dependencies: {}
impl Future for OnUpgrade { type Output = Result < Upgraded , crate :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . rx { Some (ref rx) => Pin :: new (& mut * rx . lock () . unwrap ()) . poll (cx) . map (| res | match res { Ok (Ok (upgraded)) => Ok (upgraded) , Ok (Err (err)) => Err (err) , Err (_oneshot_canceled) => { Err (crate :: Error :: new_canceled () . with (UpgradeExpected)) } }) , None => Poll :: Ready (Err (crate :: Error :: new_user_no_upgrade ())) , } } }
};
}
