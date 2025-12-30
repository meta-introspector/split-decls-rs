// Generated macro for render_helper (function)
macro_rules! Depcrate_renderrender_helper {
() => {
// Module: crate::render
// Provides: {"render_helper"}
// Dependencies: {}
# [inline] fn render_helper < 'reg : 'rc , 'rc > (ht : & 'rc HelperTemplate , registry : & 'reg Registry < 'reg > , ctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > , out : & mut dyn Output ,) -> Result < () , RenderError > { let h = Helper :: try_from_template (ht , registry , ctx , rc) ? ; debug ! ("Rendering helper: {:?}, params: {:?}, hash: {:?}" , h . name () , h . params () , h . hash ()) ; let mut call_indent_aware = | helper_def : & dyn HelperDef , rc : & mut RenderContext < 'reg , 'rc > | { let indent_directive_before = rc . get_indent_before_write () ; let content_produced_before = rc . get_content_produced () ; rc . set_content_produced (false) ; rc . set_indent_before_write (indent_directive_before || (ht . indent_before_write && rc . get_trailine_newline ()) ,) ; helper_def . call (& h , registry , ctx , rc , out) ? ; if rc . get_content_produced () { rc . set_indent_before_write (rc . get_trailine_newline ()) ; } else { rc . set_content_produced (content_produced_before) ; rc . set_indent_before_write (indent_directive_before) ; } Ok (()) } ; if let Some (ref d) = rc . get_local_helper (h . name ()) { call_indent_aware (& * * d , rc) } else { let mut helper = registry . get_or_load_helper (h . name ()) ? ; if helper . is_none () { helper = registry . get_or_load_helper (if ht . block { BLOCK_HELPER_MISSING } else { HELPER_MISSING }) ? ; } helper . ok_or_else (| | RenderErrorReason :: HelperNotFound (h . name () . to_owned ()) . into ()) . and_then (| d | call_indent_aware (& * d , rc)) } }
};
}
