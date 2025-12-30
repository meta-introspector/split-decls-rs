// Generated macro for resolve_template_arg (function)
macro_rules! Depcrateresolve_template_arg {
() => {
// Module: crate
// Provides: {"resolve_template_arg"}
// Dependencies: {}
fn resolve_template_arg < 'a > (template : & HashMap < & 'a Ident , & 'a PatType > , arg : & Ident ,) -> Option < & 'a PatType > { let id_name = arg . to_string () ; match (template . get (arg) , id_name . starts_with ('_')) { (Some (& arg) , _) => Some (arg) , (None , true) => template . get (& format_ident ! ("{}" , id_name [1 ..])) . copied () , _ => None , } }
};
}
