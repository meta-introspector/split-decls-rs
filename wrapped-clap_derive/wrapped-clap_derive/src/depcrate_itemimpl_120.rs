// Generated macro for impl_120 (impl)
macro_rules! Depcrate_itemimpl_120 {
() => {
// Module: crate::item
// Provides: {"impl_120"}
// Dependencies: {}
impl Method { pub (crate) fn new (name : Ident , args : TokenStream) -> Self { Method { name , args } } fn from_env (ident : Ident , env_var : & str) -> Result < Option < Self > , syn :: Error > { let mut lit = match env :: var (env_var) { Ok (val) => { if val . is_empty () { return Ok (None) ; } LitStr :: new (& val , ident . span ()) } Err (_) => { abort ! (ident , "cannot derive `{}` from Cargo.toml\n\n= note: {note}\n\n= help: {help}\n\n" , ident , note = format_args ! ("`{env_var}` environment variable is not set") , help = format_args ! ("use `{ident} = \"...\"` to set {ident} manually")) ; } } ; if ident == "author" { let edited = process_author_str (& lit . value ()) ; lit = LitStr :: new (& edited , lit . span ()) ; } Ok (Some (Method :: new (ident , quote ! (# lit)))) } pub (crate) fn args (& self) -> & TokenStream { & self . args } }
};
}
