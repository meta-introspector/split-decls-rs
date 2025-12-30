// Generated macro for impl_64 (impl)
macro_rules! Depcrate_hbsimpl_64 {
() => {
// Module: crate::hbs
// Provides: {"impl_64"}
// Dependencies: {}
impl HelperDef for OptionsHelper < '_ > { fn call < 'reg : 'rc , 'rc > (& self , h : & Helper < 'rc > , r : & 'reg Handlebars < 'reg > , ctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > , out : & mut dyn Output ,) -> HelperResult { if in_options (rc) { return Err (RenderErrorReason :: Other ("options blocks cannot be nested" . to_string ()) . into () ,) ; } set_in_context (rc , "__MDMAN_IN_OPTIONS" , serde_json :: Value :: Bool (true)) ; let s = self . formatter . render_options_start () ; out . write (& s) ? ; let t = match h . template () { Some (t) => t , None => { return Err (RenderErrorReason :: Other ("options block must not be empty" . to_string () ,) . into ()) ; } } ; let block = t . renders (r , ctx , rc) ? ; out . write (& block) ? ; let s = self . formatter . render_options_end () ; out . write (& s) ? ; remove_from_context (rc , "__MDMAN_IN_OPTIONS") ; Ok (()) } }
};
}
