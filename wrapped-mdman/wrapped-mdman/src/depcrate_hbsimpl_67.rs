// Generated macro for impl_67 (impl)
macro_rules! Depcrate_hbsimpl_67 {
() => {
// Module: crate::hbs
// Provides: {"impl_67"}
// Dependencies: {}
impl HelperDef for OptionHelper < '_ > { fn call < 'reg : 'rc , 'rc > (& self , h : & Helper < 'rc > , r : & 'reg Handlebars < 'reg > , gctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > , out : & mut dyn Output ,) -> HelperResult { if ! in_options (rc) { return Err (RenderErrorReason :: Other ("option must be in options block" . to_string ()) . into () ,) ; } let params = h . params () ; if params . is_empty () { return Err (RenderErrorReason :: Other ("option block must have at least one param" . to_string () ,) . into ()) ; } let params = params . iter () . map (| param | { param . value () . as_str () . ok_or_else (| | { RenderErrorReason :: Other ("option params must be strings" . to_string ()) }) . into () }) . collect :: < Result < Vec < & str > , RenderErrorReason > > () ? ; let t = match h . template () { Some (t) => t , None => { return Err (RenderErrorReason :: Other ("option block must not be empty" . to_string ()) . into () ,) ; } } ; let block = t . renders (r , gctx , rc) ? ; let block = block . replace ("\r\n" , "\n") ; let man_name = gctx . data () . get ("man_name") . expect ("expected man_name in context") . as_str () . expect ("expect man_name str") ; let option = self . formatter . render_option (& params , & block , man_name) . map_err (| e | RenderErrorReason :: Other (format ! ("option render failed: {}" , e))) ? ; out . write (& option) ? ; Ok (()) } }
};
}
