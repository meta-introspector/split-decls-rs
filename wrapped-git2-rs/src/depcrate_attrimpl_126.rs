// Generated macro for impl_126 (impl)
macro_rules! Depcrate_attrimpl_126 {
() => {
// Module: crate::attr
// Provides: {"impl_126"}
// Dependencies: {}
# [doc = " Compare two [`AttrValue`]s."] # [doc = ""] # [doc = " Note that this implementation does not differentiate between [`AttrValue::String`] and"] # [doc = " [`AttrValue::Bytes`]."] impl PartialEq for AttrValue < '_ > { fn eq (& self , other : & AttrValue < '_ >) -> bool { match (self , other) { (Self :: True , AttrValue :: True) | (Self :: False , AttrValue :: False) | (Self :: Unspecified , AttrValue :: Unspecified) => true , (AttrValue :: String (string) , AttrValue :: Bytes (bytes)) | (AttrValue :: Bytes (bytes) , AttrValue :: String (string)) => string . as_bytes () == * bytes , (AttrValue :: String (left) , AttrValue :: String (right)) => left == right , (AttrValue :: Bytes (left) , AttrValue :: Bytes (right)) => left == right , _ => false , } } }
};
}
