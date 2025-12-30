// Generated macro for AuthorityBuilder (struct)
macro_rules! Depcrate_buildAuthorityBuilder {
() => {
// Module: crate::build
// Provides: {"AuthorityBuilder"}
// Dependencies: {}
# [doc = " URI/IRI authority builder."] # [derive (Default , Debug , Clone)] struct AuthorityBuilder < 'a > { # [doc = " Host."] host : HostRepr < 'a > , # [doc = " Port."] port : PortBuilder < 'a > , # [doc = " Userinfo."] userinfo : UserinfoBuilder < 'a > , }
};
}
