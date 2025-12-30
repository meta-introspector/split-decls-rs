// Generated macro for find_partial (function)
macro_rules! Depcrate_partialfind_partial {
() => {
// Module: crate::partial
// Provides: {"find_partial"}
// Dependencies: {}
fn find_partial < 'reg : 'rc , 'rc > (rc : & RenderContext < 'reg , 'rc > , r : & 'reg Registry < 'reg > , d : & Decorator < 'rc > , name : & str ,) -> Result < Option < & 'rc Template > , RenderError > { if let Some (partial) = rc . get_partial (name) { return Ok (Some (partial)) ; } if let Some (t) = rc . get_dev_mode_template (name) { return Ok (Some (t)) ; } if let Some (t) = r . get_template (name) { return Ok (Some (t)) ; } if let Some (tpl) = d . template () { return Ok (Some (tpl)) ; } Ok (None) }
};
}
