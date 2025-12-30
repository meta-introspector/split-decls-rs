// Generated macro for extern_block_abi (function)
macro_rules! Depcrate_signaturesextern_block_abi {
() => {
// Module: crate::signatures
// Provides: {"extern_block_abi"}
// Dependencies: {}
pub (crate) fn extern_block_abi (db : & dyn DefDatabase , extern_block : ExternBlockId ,) -> Option < Symbol > { let source = extern_block . lookup (db) . source (db) ; source . value . abi () . map (| abi | { match abi . abi_string () { Some (tok) => Symbol :: intern (tok . text_without_quotes ()) , _ => sym :: C , } }) }
};
}
