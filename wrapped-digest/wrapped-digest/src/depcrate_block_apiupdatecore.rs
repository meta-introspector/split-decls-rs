// Generated macro for UpdateCore (trait)
macro_rules! Depcrate_block_apiUpdateCore {
() => {
// Module: crate::block_api
// Provides: {"UpdateCore"}
// Dependencies: {}
# [doc = " Types which consume data in blocks."] pub trait UpdateCore : BlockSizeUser { # [doc = " Update state using the provided data blocks."] fn update_blocks (& mut self , blocks : & [Block < Self >]) ; }
};
}
