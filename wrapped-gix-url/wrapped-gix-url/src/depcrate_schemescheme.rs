// Generated macro for Scheme (enum)
macro_rules! Depcrate_schemeScheme {
() => {
// Module: crate::scheme
// Provides: {"Scheme"}
// Dependencies: {}
# [doc = " A scheme or protocol for use in a [`Url`][crate::Url]."] # [doc = ""] # [doc = " It defines how to talk to a given repository."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (missing_docs)] pub enum Scheme { # [doc = " A local resource that is accessible on the current host."] File , # [doc = " A git daemon, like `File` over TCP/IP."] Git , # [doc = " Launch `git-upload-pack` through an `ssh` tunnel."] Ssh , # [doc = " Use the HTTP protocol to talk to git servers."] Http , # [doc = " Use the HTTPS protocol to talk to git servers."] Https , # [doc = " Any other protocol or transport that isn't known at compile time."] # [doc = ""] # [doc = " It's used to support plug-in transports."] Ext (String) , }
};
}
