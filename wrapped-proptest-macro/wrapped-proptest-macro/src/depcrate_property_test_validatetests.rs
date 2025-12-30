// Generated macro for tests (module)
macro_rules! Depcrate_property_test_validatetests {
() => {
// Module: crate::property_test::validate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use syn :: parse_quote ; use super :: * ; # [test] fn validate_fails_with_self_arg () { let invalids = [parse_quote ! { fn foo (self) { } } , parse_quote ! { fn foo (& self) { } } , parse_quote ! { fn foo (& mut self) { } } , parse_quote ! { fn foo (self : Self) { } } , parse_quote ! { fn foo (self : & Self) { } } , parse_quote ! { fn foo (self : & mut Self) { } } , parse_quote ! { fn foo (self : Box < Self >) { } } , parse_quote ! { fn foo (self : Rc < Self >) { } } , parse_quote ! { fn foo (self : Arc < Self >) { } } ,] ; for mut invalid in invalids { assert ! (validate (& mut invalid) . is_err ()) ; } } # [test] fn validate_fails_with_duplicate () { let mut function = parse_quote ! { fn foo (# [strategy = 1] # [strategy = 2] x : i32) { } } ; let error = validate (& mut function) . unwrap_err () ; assert ! (error . to_string () . contains ("compile_error")) ; } }
};
}
