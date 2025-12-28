macro_rules! deps {
    () => {
        SpanMap!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use expect_test :: { Expect , expect } ; use span :: { Edition , EditionedFileId , FileId } ; use syntax :: TextRange ; use syntax_bridge :: DocCommentDesugarMode ; use triomphe :: Arc ; use crate :: { fixup :: reverse_fixups , span_map :: { RealSpanMap , SpanMap } , tt , } ; fn check_leaf_eq (a : & tt :: Leaf , b : & tt :: Leaf) -> bool { match (a , b) { (tt :: Leaf :: Literal (a) , tt :: Leaf :: Literal (b)) => a . symbol == b . symbol , (tt :: Leaf :: Punct (a) , tt :: Leaf :: Punct (b)) => a . char == b . char , (tt :: Leaf :: Ident (a) , tt :: Leaf :: Ident (b)) => a . sym == b . sym , _ => false , } } fn check_subtree_eq (a : & tt :: TopSubtree , b : & tt :: TopSubtree) -> bool { let a = a . view () . as_token_trees () . flat_tokens () ; let b = b . view () . as_token_trees () . flat_tokens () ; a . len () == b . len () && std :: iter :: zip (a , b) . all (| (a , b) | check_tt_eq (a , b)) } fn check_tt_eq (a : & tt :: TokenTree , b : & tt :: TokenTree) -> bool { match (a , b) { (tt :: TokenTree :: Leaf (a) , tt :: TokenTree :: Leaf (b)) => check_leaf_eq (a , b) , (tt :: TokenTree :: Subtree (a) , tt :: TokenTree :: Subtree (b)) => { a . delimiter . kind == b . delimiter . kind } _ => false , } } # [track_caller] fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str , mut expect : Expect) { let parsed = syntax :: SourceFile :: parse (ra_fixture , span :: Edition :: CURRENT) ; let span_map = SpanMap :: RealSpanMap (Arc :: new (RealSpanMap :: absolute (EditionedFileId :: new (FileId :: from_raw (0) , Edition :: CURRENT ,)))) ; let fixups = super :: fixup_syntax (span_map . as_ref () , & parsed . syntax_node () , span_map . span_for_range (TextRange :: empty (0 . into ())) , DocCommentDesugarMode :: Mbe ,) ; let mut tt = syntax_bridge :: syntax_node_to_token_tree_modified (& parsed . syntax_node () , span_map . as_ref () , fixups . append , fixups . remove , span_map . span_for_range (TextRange :: empty (0 . into ())) , DocCommentDesugarMode :: Mbe ,) ; let actual = format ! ("{tt}\n") ; expect . indent (false) ; expect . assert_eq (& actual) ; let (parse , _) = syntax_bridge :: token_tree_to_syntax_node (& tt , syntax_bridge :: TopEntryPoint :: MacroItems , & mut | _ | parser :: Edition :: CURRENT , parser :: Edition :: CURRENT ,) ; assert ! (parse . errors () . is_empty () , "parse has syntax errors. parse tree:\n{:#?}" , parse . syntax_node ()) ; for x in tt . token_trees () . flat_tokens () { match x { :: tt :: TokenTree :: Leaf (:: tt :: Leaf :: Punct (punct)) => { assert ! (! matches ! (punct . char , '{' | '}' | '(' | ')' | '[' | ']')) } _ => () , } } reverse_fixups (& mut tt , & fixups . undo_info) ; let original_as_tt = syntax_bridge :: syntax_node_to_token_tree (& parsed . syntax_node () , span_map . as_ref () , span_map . span_for_range (TextRange :: empty (0 . into ())) , DocCommentDesugarMode :: Mbe ,) ; assert ! (check_subtree_eq (& tt , & original_as_tt) , "different token tree:\n{tt:?}\n\n{original_as_tt:?}") ; } # [test] fn just_for_token () { check (r#"
fn foo() {
    for
}
"# , expect ! [[r#"
fn foo () {for _ in __ra_fixup {}}
"#]] ,) } # [test] fn for_no_iter_pattern () { check (r#"
fn foo() {
    for {}
}
"# , expect ! [[r#"
fn foo () {for _ in __ra_fixup {}}
"#]] ,) } # [test] fn for_no_body () { check (r#"
fn foo() {
    for bar in qux
}
"# , expect ! [[r#"
fn foo () {for bar in qux {}}
"#]] ,) } # [test] fn for_no_pat () { check (r#"
fn foo() {
    for in qux {

    }
}
"# , expect ! [[r#"
fn foo () {__ra_fixup}
"#]] ,) } # [test] fn match_no_expr_no_arms () { check (r#"
fn foo() {
    match
}
"# , expect ! [[r#"
fn foo () {match __ra_fixup {}}
"#]] ,) } # [test] fn match_expr_no_arms () { check (r#"
fn foo() {
    match it {

    }
}
"# , expect ! [[r#"
fn foo () {match it {}}
"#]] ,) } # [test] fn match_no_expr () { check (r#"
fn foo() {
    match {
        _ => {}
    }
}
"# , expect ! [[r#"
fn foo () {match __ra_fixup {}}
"#]] ,) } # [test] fn incomplete_field_expr_1 () { check (r#"
fn foo() {
    a.
}
"# , expect ! [[r#"
fn foo () {a . __ra_fixup}
"#]] ,) } # [test] fn incomplete_field_expr_2 () { check (r#"
fn foo() {
    a.;
}
"# , expect ! [[r#"
fn foo () {a . __ra_fixup ;}
"#]] ,) } # [test] fn incomplete_field_expr_3 () { check (r#"
fn foo() {
    a.;
    bar();
}
"# , expect ! [[r#"
fn foo () {a . __ra_fixup ; bar () ;}
"#]] ,) } # [test] fn incomplete_let () { check (r#"
fn foo() {
    let it = a
}
"# , expect ! [[r#"
fn foo () {let it = a ;}
"#]] ,) } # [test] fn incomplete_field_expr_in_let () { check (r#"
fn foo() {
    let it = a.
}
"# , expect ! [[r#"
fn foo () {let it = a . __ra_fixup ;}
"#]] ,) } # [test] fn field_expr_before_call () { check (r#"
fn foo() {
    a.b
    bar();
}
"# , expect ! [[r#"
fn foo () {a . b ; bar () ;}
"#]] ,) } # [test] fn extraneous_comma () { check (r#"
fn foo() {
    bar(,);
}
"# , expect ! [[r#"
fn foo () {__ra_fixup ;}
"#]] ,) } # [test] fn fixup_if_1 () { check (r#"
fn foo() {
    if a
}
"# , expect ! [[r#"
fn foo () {if a {}}
"#]] ,) } # [test] fn fixup_if_2 () { check (r#"
fn foo() {
    if
}
"# , expect ! [[r#"
fn foo () {if __ra_fixup {}}
"#]] ,) } # [test] fn fixup_if_3 () { check (r#"
fn foo() {
    if {}
}
"# , expect ! [[r#"
fn foo () {if __ra_fixup {} {}}
"#]] ,) } # [test] fn fixup_while_1 () { check (r#"
fn foo() {
    while
}
"# , expect ! [[r#"
fn foo () {while __ra_fixup {}}
"#]] ,) } # [test] fn fixup_while_2 () { check (r#"
fn foo() {
    while foo
}
"# , expect ! [[r#"
fn foo () {while foo {}}
"#]] ,) } # [test] fn fixup_while_3 () { check (r#"
fn foo() {
    while {}
}
"# , expect ! [[r#"
fn foo () {while __ra_fixup {}}
"#]] ,) } # [test] fn fixup_loop () { check (r#"
fn foo() {
    loop
}
"# , expect ! [[r#"
fn foo () {loop {}}
"#]] ,) } # [test] fn fixup_path () { check (r#"
fn foo() {
    path::
}
"# , expect ! [[r#"
fn foo () {path :: __ra_fixup}
"#]] ,) } # [test] fn fixup_record_ctor_field () { check (r#"
fn foo() {
    R { f: }
}
"# , expect ! [[r#"
fn foo () {R {f : __ra_fixup}}
"#]] ,) } # [test] fn no_fixup_record_ctor_field () { check (r#"
fn foo() {
    R { f: a }
}
"# , expect ! [[r#"
fn foo () {R {f : a}}
"#]] ,) } # [test] fn fixup_arg_list () { check (r#"
fn foo() {
    foo(a
}
"# , expect ! [[r#"
fn foo () {foo (a)}
"#]] ,) ; check (r#"
fn foo() {
    bar.foo(a
}
"# , expect ! [[r#"
fn foo () {bar . foo (a)}
"#]] ,) ; } # [test] fn fixup_closure () { check (r#"
fn foo() {
    ||
}
"# , expect ! [[r#"
fn foo () {|| __ra_fixup}
"#]] ,) ; } # [test] fn fixup_regression_ () { check (r#"
fn foo() {
    {}
    {}
}
"# , expect ! [[r#"
fn foo () {{} {}}
"#]] ,) ; } }
    };
}

tests!()