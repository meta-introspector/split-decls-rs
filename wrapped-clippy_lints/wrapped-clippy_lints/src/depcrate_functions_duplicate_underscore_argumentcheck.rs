// Generated macro for check (function)
macro_rules! Depcrate_functions_duplicate_underscore_argumentcheck {
() => {
// Module: crate::functions::duplicate_underscore_argument
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , fn_kind : FnKind < '_ >) { let mut registered_names : FxHashMap < String , Span > = FxHashMap :: default () ; for arg in & fn_kind . decl () . inputs { if let PatKind :: Ident (_ , ident , None) = arg . pat . kind { let arg_name = ident . to_string () ; if let Some (arg_name) = arg_name . strip_prefix ('_') { if let Some (correspondence) = registered_names . get (arg_name) { span_lint (cx , DUPLICATE_UNDERSCORE_ARGUMENT , * correspondence , format ! ("`{arg_name}` already exists, having another argument having almost the same \
                                 name makes code comprehension and documentation more difficult") ,) ; } } else { registered_names . insert (arg_name , arg . pat . span) ; } } } }
};
}
