// Generated macro for test (module)
macro_rules! Depcrate_decorators_inlinetest {
() => {
// Module: crate::decorators::inline
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: context :: Context ; use crate :: registry :: Registry ; use crate :: render :: { Evaluable , RenderContext } ; use crate :: template :: Template ; # [test] fn test_inline () { let t0 = Template :: compile ("{{#*inline \"hello\"}}the hello world inline partial.{{/inline}}") . ok () . unwrap () ; let hbs = Registry :: new () ; let ctx = Context :: null () ; let mut rc = RenderContext :: new (None) ; t0 . elements [0] . eval (& hbs , & ctx , & mut rc) . unwrap () ; assert ! (rc . get_partial ("hello") . is_some ()) ; } }
};
}
