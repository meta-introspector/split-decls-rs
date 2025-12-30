// Generated macro for Name (enum)
macro_rules! Depcrate_remoteName {
() => {
// Module: crate::remote
// Provides: {"Name"}
// Dependencies: {}
# [doc = " The name of a remote, either interpreted as symbol like `origin` or as url as returned by [`Remote::name()`][crate::Remote::name()]."] # [derive (Debug , PartialEq , Eq , Clone , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Name < 'repo > { # [doc = " A symbolic name, like `origin`."] # [doc = " Note that it has not necessarily been validated yet."] Symbol (Cow < 'repo , str >) , # [doc = " A url pointing to the remote host directly."] Url (Cow < 'repo , BStr >) , }
};
}
