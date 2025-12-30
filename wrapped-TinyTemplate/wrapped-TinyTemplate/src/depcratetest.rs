// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [derive (Serialize)] struct Context { name : String , } static TEMPLATE : & 'static str = "Hello {name}!" ; # [test] pub fn test_set_default_formatter () { let mut tt = TinyTemplate :: new () ; tt . add_template ("hello" , TEMPLATE) . unwrap () ; tt . set_default_formatter (& format_unescaped) ; let context = Context { name : "<World>" . to_string () , } ; let rendered = tt . render ("hello" , & context) . unwrap () ; assert_eq ! (rendered , "Hello <World>!") } }
};
}
