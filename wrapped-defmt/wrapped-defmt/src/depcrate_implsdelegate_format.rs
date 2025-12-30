// Generated macro for delegate_format (macro)
macro_rules! Depcrate_implsdelegate_format {
() => {
// Module: crate::impls
// Provides: {"delegate_format"}
// Dependencies: {}
macro_rules ! delegate_format { ($ ty : ty , $ self_ : ident , $ val : expr) => { # [inline] fn format (&$ self_ , fmt : Formatter) { <$ ty as Format >:: format ($ val , fmt) } # [inline] fn _format_tag () -> Str { <$ ty as Format >:: _format_tag () } # [inline] fn _format_data (&$ self_) { <$ ty as Format >:: _format_data ($ val) } } ; }
};
}
