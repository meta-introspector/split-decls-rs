// Generated macro for tests (module)
macro_rules! Depcrate_enumeratedtests {
() => {
// Module: crate::enumerated
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [allow (clippy :: unwrap_used)] mod tests { use super :: DeriveEnumerated ; use syn :: parse_quote ; # [doc = " X.509 `CRLReason`."] # [test] fn crlreason_example () { let input = parse_quote ! { # [repr (u32)] pub enum CrlReason { Unspecified = 0 , KeyCompromise = 1 , CaCompromise = 2 , AffiliationChanged = 3 , Superseded = 4 , CessationOfOperation = 5 , CertificateHold = 6 , RemoveFromCrl = 8 , PrivilegeWithdrawn = 9 , AaCompromised = 10 , } } ; let ir = DeriveEnumerated :: new (input) . unwrap () ; assert_eq ! (ir . ident , "CrlReason") ; assert_eq ! (ir . repr , "u32") ; assert_eq ! (ir . variants . len () , 10) ; let unspecified = & ir . variants [0] ; assert_eq ! (unspecified . ident , "Unspecified") ; assert_eq ! (unspecified . discriminant . to_string () , "0") ; let key_compromise = & ir . variants [1] ; assert_eq ! (key_compromise . ident , "KeyCompromise") ; assert_eq ! (key_compromise . discriminant . to_string () , "1") ; let key_compromise = & ir . variants [2] ; assert_eq ! (key_compromise . ident , "CaCompromise") ; assert_eq ! (key_compromise . discriminant . to_string () , "2") ; } }
};
}
