// Generated macro for macro_backtrace (function)
macro_rules! Depcrate_macrosmacro_backtrace {
() => {
// Module: crate::macros
// Provides: {"macro_backtrace"}
// Dependencies: {}
# [doc = " Returns an iterator of macro expansions that created the given span."] # [doc = " Note that desugaring expansions are skipped."] pub fn macro_backtrace (span : Span) -> impl Iterator < Item = MacroCall > { expn_backtrace (span) . filter_map (| (expn , data) | match data { ExpnData { kind : ExpnKind :: Macro (kind , _) , macro_def_id : Some (def_id) , call_site : span , .. } => Some (MacroCall { def_id , kind , expn , span , }) , _ => None , }) }
};
}
