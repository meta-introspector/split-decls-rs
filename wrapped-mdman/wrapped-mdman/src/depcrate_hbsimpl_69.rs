// Generated macro for impl_69 (impl)
macro_rules! Depcrate_hbsimpl_69 {
() => {
// Module: crate::hbs
// Provides: {"impl_69"}
// Dependencies: {}
impl HelperDef for ManLinkHelper < '_ > { fn call < 'reg : 'rc , 'rc > (& self , h : & Helper < 'rc > , _r : & 'reg Handlebars < 'reg > , _ctx : & 'rc Context , _rc : & mut RenderContext < 'reg , 'rc > , out : & mut dyn Output ,) -> HelperResult { let params = h . params () ; if params . len () != 2 { return Err (RenderErrorReason :: Other ("{{man}} must have two arguments" . to_string ()) . into () ,) ; } let name = params [0] . value () . as_str () . ok_or_else (| | { RenderErrorReason :: Other ("man link name must be a string" . to_string ()) }) ? ; let section = params [1] . value () . as_u64 () . ok_or_else (| | { RenderErrorReason :: Other ("man link section must be an integer" . to_string ()) }) ? ; let section = u8 :: try_from (section) . map_err (| _e | RenderErrorReason :: Other ("section number too large" . to_string ())) ? ; let link = self . formatter . linkify_man_to_md (name , section) . map_err (| e | RenderErrorReason :: Other (format ! ("failed to linkify man: {}" , e))) ? ; out . write (& link) ? ; Ok (()) } }
};
}
