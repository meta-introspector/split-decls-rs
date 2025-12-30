// Generated macro for impl_194 (impl)
macro_rules! Depcrate_dictionaryimpl_194 {
() => {
// Module: crate::dictionary
// Provides: {"impl_194"}
// Dependencies: {}
# [doc = " Convenience mutation methods."] impl < KeyType : Message , ObjectType : Message > NSMutableDictionary < KeyType , ObjectType > { # [doc = " Inserts a key-value pair into the dictionary."] # [doc = ""] # [doc = " If the dictionary did not have this key present, the value is"] # [doc = " inserted. If the dictionary already had this key present, the value"] # [doc = " and the key is updated."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use objc2_foundation::{ns_string, NSMutableDictionary, NSObject};"] # [doc = ""] # [doc = " let dict = NSMutableDictionary::new();"] # [doc = " dict.insert(ns_string!(\"key\"), &*NSObject::new());"] # [doc = " ```"] # [cfg (feature = "NSObject")] # [doc (alias = "setObject:forKey:")] # [inline] pub fn insert < CopiedKey > (& self , key : & CopiedKey , object : & ObjectType) where CopiedKey : Message + NSCopying + CopyingHelper < Result = KeyType > , { let key = ProtocolObject :: from_ref (key) ; unsafe { self . setObject_forKey (object , key) } ; } }
};
}
