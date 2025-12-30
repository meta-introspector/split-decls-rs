// Generated macro for BasicBlock (struct)
macro_rules! Depcrate_mirBasicBlock {
() => {
// Module: crate::mir
// Provides: {"BasicBlock"}
// Dependencies: {}
# [derive (Debug , Default , Clone , PartialEq , Eq)] pub struct BasicBlock < 'db > { # [doc = " List of statements in this block."] pub statements : Vec < Statement < 'db > > , # [doc = " Terminator for this block."] # [doc = ""] # [doc = " N.B., this should generally ONLY be `None` during construction."] # [doc = " Therefore, you should generally access it via the"] # [doc = " `terminator()` or `terminator_mut()` methods. The only"] # [doc = " exception is that certain passes, such as `simplify_cfg`, swap"] # [doc = " out the terminator temporarily with `None` while they continue"] # [doc = " to recurse over the set of basic blocks."] pub terminator : Option < Terminator < 'db > > , # [doc = " If true, this block lies on an unwind path. This is used"] # [doc = " during codegen where distinct kinds of basic blocks may be"] # [doc = " generated (particularly for MSVC cleanup). Unwind blocks must"] # [doc = " only branch to other unwind blocks."] pub is_cleanup : bool , }
};
}
