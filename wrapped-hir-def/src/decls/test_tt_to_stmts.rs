macro_rules! test_tt_to_stmts {
    () => {
        # [test] fn test_tt_to_stmts () { check (r#"
macro_rules! m {
    () => {
        let a = 0;
        a = 10 + 1;
        a
    }
}

fn f() -> i32 {
    m!/*+tree*/{}
}
"# , expect ! [[r#"
macro_rules! m {
    () => {
        let a = 0;
        a = 10 + 1;
        a
    }
}

fn f() -> i32 {
    let a = 0;
    a = 10+1;
    a
// MACRO_STMTS@0..15
//   LET_STMT@0..7
//     LET_KW@0..3 "let"
//     IDENT_PAT@3..4
//       NAME@3..4
//         IDENT@3..4 "a"
//     EQ@4..5 "="
//     LITERAL@5..6
//       INT_NUMBER@5..6 "0"
//     SEMICOLON@6..7 ";"
//   EXPR_STMT@7..14
//     BIN_EXPR@7..13
//       PATH_EXPR@7..8
//         PATH@7..8
//           PATH_SEGMENT@7..8
//             NAME_REF@7..8
//               IDENT@7..8 "a"
//       EQ@8..9 "="
//       BIN_EXPR@9..13
//         LITERAL@9..11
//           INT_NUMBER@9..11 "10"
//         PLUS@11..12 "+"
//         LITERAL@12..13
//           INT_NUMBER@12..13 "1"
//     SEMICOLON@13..14 ";"
//   PATH_EXPR@14..15
//     PATH@14..15
//       PATH_SEGMENT@14..15
//         NAME_REF@14..15
//           IDENT@14..15 "a"

}
"#]] ,) ; }
    };
}

test_tt_to_stmts!();