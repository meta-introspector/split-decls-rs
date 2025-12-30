// Generated macro for Nid (struct)
macro_rules! Depcrate_nidNid {
() => {
// Module: crate::nid
// Provides: {"Nid"}
// Dependencies: {}
# [doc = " A numerical identifier for an OpenSSL object."] # [doc = ""] # [doc = " Objects in OpenSSL can have a short name, a long name, and"] # [doc = " a numerical identifier (NID). For convenience, objects"] # [doc = " are usually represented in source code using these numeric"] # [doc = " identifiers."] # [doc = ""] # [doc = " Users should generally not need to create new `Nid`s."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " To view the integer representation of a `Nid`:"] # [doc = ""] # [doc = " ```"] # [doc = " use openssl::nid::Nid;"] # [doc = ""] # [doc = " assert!(Nid::AES_256_GCM.as_raw() == 901);"] # [doc = " ```"] # [doc = ""] # [doc = " # External Documentation"] # [doc = ""] # [doc = " The following documentation provides context about `Nid`s and their usage"] # [doc = " in OpenSSL."] # [doc = ""] # [doc = " - [Obj_nid2obj](https://docs.openssl.org/master/man3/OBJ_create/)"] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub struct Nid (c_int) ;
};
}
