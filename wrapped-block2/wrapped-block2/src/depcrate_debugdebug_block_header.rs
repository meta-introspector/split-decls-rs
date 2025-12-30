// Generated macro for debug_block_header (function)
macro_rules! Depcrate_debugdebug_block_header {
() => {
// Module: crate::debug
// Provides: {"debug_block_header"}
// Dependencies: {}
pub (crate) fn debug_block_header (header : & BlockHeader , f : & mut DebugStruct < '_ , '_ >) { f . field ("isa" , & Isa (header . isa)) ; f . field ("flags" , & header . flags) ; f . field ("reserved" , & header . reserved) ; f . field ("invoke" , & header . invoke) ; f . field ("descriptor" , & BlockDescriptorHelper { has_copy_dispose : header . flags . has (BlockFlags :: BLOCK_HAS_COPY_DISPOSE) , has_signature : header . flags . has (BlockFlags :: BLOCK_HAS_SIGNATURE) , descriptor : header . descriptor , } ,) ; }
};
}
