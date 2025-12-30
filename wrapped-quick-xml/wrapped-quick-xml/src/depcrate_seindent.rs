// Generated macro for Indent (enum)
macro_rules! Depcrate_seIndent {
() => {
// Module: crate::se
// Provides: {"Indent"}
// Dependencies: {}
pub (crate) enum Indent < 'i > { # [doc = " No indent should be written before the element"] None , # [doc = " The specified indent should be written. The type owns the buffer with indent"] Owned (Indentation) , # [doc = " The specified indent should be written. The type borrows buffer with indent"] # [doc = " from its owner"] Borrow (& 'i mut Indentation) , }
};
}
