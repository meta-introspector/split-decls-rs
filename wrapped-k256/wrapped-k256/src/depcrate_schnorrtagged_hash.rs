// Generated macro for tagged_hash (function)
macro_rules! Depcrate_schnorrtagged_hash {
() => {
// Module: crate::schnorr
// Provides: {"tagged_hash"}
// Dependencies: {}
fn tagged_hash (tag : & [u8]) -> Sha256 { let tag_hash = Sha256 :: digest (tag) ; let mut digest = Sha256 :: new () ; digest . update (tag_hash) ; digest . update (tag_hash) ; digest }
};
}
