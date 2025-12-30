// Generated macro for CowBytesWrap (struct)
macro_rules! Depcrate_serde_borrow_de_utilsCowBytesWrap {
() => {
// Module: crate::serde_borrow_de_utils
// Provides: {"CowBytesWrap"}
// Dependencies: {}
# [derive (Deserialize)] # [serde (transparent)] # [allow (clippy :: exhaustive_structs)] # [derive (Debug)] pub struct CowBytesWrap < 'data > (# [serde (borrow)] pub Cow < 'data , [u8] >) ;
};
}
