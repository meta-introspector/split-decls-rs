// Generated macro for PackContext (struct)
macro_rules! Depcrate_index_verifyPackContext {
() => {
// Module: crate::index::verify
// Provides: {"PackContext"}
// Dependencies: {}
# [doc = " Information to allow verifying the integrity of an index with the help of its corresponding pack."] pub struct PackContext < 'a , F > { # [doc = " The pack data file itself."] pub data : & 'a crate :: data :: File , # [doc = " The options further configuring the pack traversal and verification"] pub options : integrity :: Options < F > , }
};
}
