// Generated macro for tests (module)
macro_rules! Depcrate_navigation_targettests {
() => {
// Module: crate::navigation_target
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use expect_test :: expect ; use crate :: { Query , fixture } ; # [test] fn test_nav_for_symbol () { let (analysis , _) = fixture :: file (r#"
enum FooInner { }
fn foo() { enum FooInner { } }
"# ,) ; let navs = analysis . symbol_search (Query :: new ("FooInner" . to_owned ()) , ! 0) . unwrap () ; expect ! [[r#"
            [
                NavigationTarget {
                    file_id: FileId(
                        0,
                    ),
                    full_range: 0..17,
                    focus_range: 5..13,
                    name: "FooInner",
                    kind: Enum,
                    description: "enum FooInner",
                },
                NavigationTarget {
                    file_id: FileId(
                        0,
                    ),
                    full_range: 29..46,
                    focus_range: 34..42,
                    name: "FooInner",
                    kind: Enum,
                    container_name: "foo",
                    description: "enum FooInner",
                },
            ]
        "#]] . assert_debug_eq (& navs) ; } # [test] fn test_world_symbols_are_case_sensitive () { let (analysis , _) = fixture :: file (r#"
fn foo() {}
struct Foo;
"# ,) ; let navs = analysis . symbol_search (Query :: new ("foo" . to_owned ()) , ! 0) . unwrap () ; assert_eq ! (navs . len () , 2) } # [test] fn test_ensure_hidden_symbols_are_not_returned () { let (analysis , _) = fixture :: file (r#"
fn foo() {}
struct Foo;
static __FOO_CALLSITE: () = ();
"# ,) ; let navs = analysis . symbol_search (Query :: new ("foo" . to_owned ()) , ! 0) . unwrap () ; assert_eq ! (navs . len () , 2) ; let navs = analysis . symbol_search (Query :: new ("_foo" . to_owned ()) , ! 0) . unwrap () ; assert_eq ! (navs . len () , 0) ; let query = Query :: new ("__foo" . to_owned ()) ; let navs = analysis . symbol_search (query , ! 0) . unwrap () ; assert_eq ! (navs . len () , 1) ; } }
};
}
