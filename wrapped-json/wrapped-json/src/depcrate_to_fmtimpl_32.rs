// Generated macro for impl_32 (impl)
macro_rules! Depcrate_to_fmtimpl_32 {
() => {
// Module: crate::to_fmt
// Provides: {"impl_32"}
// Dependencies: {}
impl < W > fmt :: Debug for Formatter < W > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Formatter") . field ("is_internally_tagged" , & self . is_internally_tagged) . field ("is_current_depth_empty" , & self . is_current_depth_empty) . field ("is_text_quoted" , & self . is_text_quoted) . field ("err" , & self . err) . field ("text_handler" , & self . text_handler . as_ref () . map (| _ | ())) . finish () } }
};
}
