// Generated macro for BagType (enum)
macro_rules! Depcrate_bag_typeBagType {
() => {
// Module: crate::bag_type
// Provides: {"BagType"}
// Dependencies: {}
# [doc = " Indicates the type of content."] # [derive (Copy , Clone , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] pub enum BagType { # [doc = " Plain data content type"] Key , # [doc = " Signed-data content type"] Pkcs8 , # [doc = " Enveloped-data content type"] Cert , # [doc = " Signed-and-enveloped-data content type"] Crl , # [doc = " Digested-data content type"] Secret , # [doc = " Encrypted-data content type"] SafeContents , }
};
}
