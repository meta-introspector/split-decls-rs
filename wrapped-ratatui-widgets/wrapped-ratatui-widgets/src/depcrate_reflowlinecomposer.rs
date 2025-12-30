// Generated macro for LineComposer (trait)
macro_rules! Depcrate_reflowLineComposer {
() => {
// Module: crate::reflow
// Provides: {"LineComposer"}
// Dependencies: {}
# [doc = " A state machine to pack styled symbols into lines."] # [doc = " Cannot implement it as Iterator since it yields slices of the internal buffer (need streaming"] # [doc = " iterators for that)."] pub trait LineComposer < 'a > { fn next_line < 'lend > (& 'lend mut self) -> Option < WrappedLine < 'lend , 'a > > ; }
};
}
