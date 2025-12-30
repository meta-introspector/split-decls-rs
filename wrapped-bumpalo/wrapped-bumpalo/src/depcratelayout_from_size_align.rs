// Generated macro for layout_from_size_align (function)
macro_rules! Depcratelayout_from_size_align {
() => {
// Module: crate
// Provides: {"layout_from_size_align"}
// Dependencies: {}
# [doc = " Wrapper around `Layout::from_size_align` that adds debug assertions."] # [inline] fn layout_from_size_align (size : usize , align : usize) -> Result < Layout , AllocErr > { Layout :: from_size_align (size , align) . map_err (| _ | AllocErr) }
};
}
