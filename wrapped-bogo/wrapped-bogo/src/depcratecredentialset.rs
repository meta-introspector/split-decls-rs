// Generated macro for CredentialSet (struct)
macro_rules! DepcrateCredentialSet {
() => {
// Module: crate
// Provides: {"CredentialSet"}
// Dependencies: {}
# [derive (Debug , Default)] struct CredentialSet { default : Credential , additional : Vec < Credential > , # [doc = " Some(-1) means `default`, otherwise index into `additional`"] expect_selected : Option < isize > , }
};
}
