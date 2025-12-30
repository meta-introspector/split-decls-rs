// Generated macro for get_in_block_params (function)
macro_rules! Depcrate_contextget_in_block_params {
() => {
// Module: crate::context
// Provides: {"get_in_block_params"}
// Dependencies: {}
fn get_in_block_params < 'a > (block_contexts : & 'a VecDeque < BlockContext < '_ > > , p : & str ,) -> Option < (& 'a BlockParamHolder , & 'a Vec < String >) > { for bc in block_contexts { let v = bc . get_block_param (p) ; if v . is_some () { return v . map (| v | (v , bc . base_path ())) ; } } None }
};
}
