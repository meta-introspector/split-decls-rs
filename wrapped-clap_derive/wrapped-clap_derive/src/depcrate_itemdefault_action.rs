// Generated macro for default_action (function)
macro_rules! Depcrate_itemdefault_action {
() => {
// Module: crate::item
// Provides: {"default_action"}
// Dependencies: {}
fn default_action (field_type : & Type , span : Span) -> Method { let ty = Ty :: from_syn_ty (field_type) ; let args = match * ty { Ty :: Vec | Ty :: OptionVec | Ty :: VecVec | Ty :: OptionVecVec => { quote_spanned ! { span => clap :: ArgAction :: Append } } Ty :: Option | Ty :: OptionOption => { quote_spanned ! { span => clap :: ArgAction :: Set } } _ => { if is_simple_ty (field_type , "bool") { quote_spanned ! { span => clap :: ArgAction :: SetTrue } } else { quote_spanned ! { span => clap :: ArgAction :: Set } } } } ; let func = Ident :: new ("action" , span) ; Method :: new (func , args) }
};
}
