// Generated macro for text_edit_from_self_param (function)
macro_rules! Depcrate_renametext_edit_from_self_param {
() => {
// Module: crate::rename
// Provides: {"text_edit_from_self_param"}
// Dependencies: {}
fn text_edit_from_self_param (self_param : & ast :: SelfParam , new_name : String) -> Option < TextEdit > { let mut replacement_text = new_name ; replacement_text . push_str (": ") ; if self_param . amp_token () . is_some () { replacement_text . push ('&') ; } if let Some (lifetime) = self_param . lifetime () { write ! (replacement_text , "{lifetime} ") . unwrap () ; } if self_param . amp_token () . and (self_param . mut_token ()) . is_some () { replacement_text . push_str ("mut ") ; } replacement_text . push_str ("Self") ; Some (TextEdit :: replace (self_param . syntax () . text_range () , replacement_text)) }
};
}
