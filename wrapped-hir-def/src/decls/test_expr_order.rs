macro_rules! test_expr_order {
    () => {
        # [test] fn test_expr_order () { check (r#"
macro_rules! m {
    ($ i:expr) => { fn bar() { $ i * 3; } }
}
// +tree
m! { 1 + 2 }
"# , expect ! [[r#"
macro_rules! m {
    ($ i:expr) => { fn bar() { $ i * 3; } }
}
fn bar() {
    (1+2)*3;
}
// MACRO_ITEMS@0..17
//   FN@0..17
//     FN_KW@0..2 "fn"
//     NAME@2..5
//       IDENT@2..5 "bar"
//     PARAM_LIST@5..7
//       L_PAREN@5..6 "("
//       R_PAREN@6..7 ")"
//     BLOCK_EXPR@7..17
//       STMT_LIST@7..17
//         L_CURLY@7..8 "{"
//         EXPR_STMT@8..16
//           BIN_EXPR@8..15
//             PAREN_EXPR@8..13
//               L_PAREN@8..9 "("
//               BIN_EXPR@9..12
//                 LITERAL@9..10
//                   INT_NUMBER@9..10 "1"
//                 PLUS@10..11 "+"
//                 LITERAL@11..12
//                   INT_NUMBER@11..12 "2"
//               R_PAREN@12..13 ")"
//             STAR@13..14 "*"
//             LITERAL@14..15
//               INT_NUMBER@14..15 "3"
//           SEMICOLON@15..16 ";"
//         R_CURLY@16..17 "}"

"#]] ,) }
    };
}

test_expr_order!();