macro_rules! test_expr_after_path_colons {
    () => {
        # [test] fn test_expr_after_path_colons () { check (r#"
macro_rules! m {
    ($k:expr) => { fn f() { K::$k; } }
}
// +tree +errors
m!(C("0"));
"# , expect ! [[r#"
macro_rules! m {
    ($k:expr) => { fn f() { K::$k; } }
}
/* parse error: expected identifier, `self`, `super`, `crate`, or `Self` */
/* parse error: expected SEMICOLON */
/* parse error: expected SEMICOLON */
/* parse error: expected expression, item or let statement */
fn f() {
    K::(C("0"));
}
// MACRO_ITEMS@0..19
//   FN@0..19
//     FN_KW@0..2 "fn"
//     NAME@2..3
//       IDENT@2..3 "f"
//     PARAM_LIST@3..5
//       L_PAREN@3..4 "("
//       R_PAREN@4..5 ")"
//     BLOCK_EXPR@5..19
//       STMT_LIST@5..19
//         L_CURLY@5..6 "{"
//         EXPR_STMT@6..10
//           PATH_EXPR@6..10
//             PATH@6..10
//               PATH@6..7
//                 PATH_SEGMENT@6..7
//                   NAME_REF@6..7
//                     IDENT@6..7 "K"
//               COLON2@7..9 "::"
//               PATH_SEGMENT@9..10
//                 ERROR@9..10
//                   L_PAREN@9..10 "("
//         EXPR_STMT@10..16
//           CALL_EXPR@10..16
//             PATH_EXPR@10..11
//               PATH@10..11
//                 PATH_SEGMENT@10..11
//                   NAME_REF@10..11
//                     IDENT@10..11 "C"
//             ARG_LIST@11..16
//               L_PAREN@11..12 "("
//               LITERAL@12..15
//                 STRING@12..15 "\"0\""
//               R_PAREN@15..16 ")"
//         ERROR@16..17
//           R_PAREN@16..17 ")"
//         SEMICOLON@17..18 ";"
//         R_CURLY@18..19 "}"

"#]] ,) ; }
    };
}

test_expr_after_path_colons!()