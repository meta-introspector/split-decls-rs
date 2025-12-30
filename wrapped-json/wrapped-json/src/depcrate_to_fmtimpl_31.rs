// Generated macro for impl_31 (impl)
macro_rules! Depcrate_to_fmtimpl_31 {
() => {
// Module: crate::to_fmt
// Provides: {"impl_31"}
// Dependencies: {}
impl < W > Formatter < W > { pub fn new (out : W) -> Self { Formatter { is_internally_tagged : false , is_current_depth_empty : true , is_text_quoted : true , text_handler : None , err : None , out , } } fn err (& mut self , e : Error) -> sval :: Error { self . err = Some (e) ; sval :: Error :: new () } }
};
}
