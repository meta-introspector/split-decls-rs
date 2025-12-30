// Generated macro for CowWrap (struct)
macro_rules! Depcrate_serde_borrow_de_utilsCowWrap {
() => {
// Module: crate::serde_borrow_de_utils
// Provides: {"CowWrap"}
// Dependencies: {}
# [derive (Deserialize)] # [serde (transparent)] # [allow (clippy :: exhaustive_structs)] # [derive (Debug)] pub struct CowWrap < 'data > (# [serde (borrow)] pub Cow < 'data , str >) ;
};
}
