// Generated macro for impl_173 (impl)
macro_rules! Depcrate_blob_platformimpl_173 {
() => {
// Module: crate::blob::platform
// Provides: {"impl_173"}
// Dependencies: {}
impl PartialEq for CacheKey { fn eq (& self , other : & Self) -> bool { match (self . use_id , other . use_id) { (false , false) => self . location . eq (& other . location) , (true , true) => self . id . eq (& other . id) && self . is_link . eq (& other . is_link) , _ => false , } } }
};
}
