// Generated macro for expr_2021 (function)
macro_rules! Depcrate_testsexpr_2021 {
() => {
// Module: crate::tests
// Provides: {"expr_2021"}
// Dependencies: {}
# [test] fn expr_2021 () { check (Edition :: Edition2024 , Edition :: Edition2024 , r#"
($($e:expr),* $(,)?) => {
    $($e);* ;
};
"# , r#"
    _,
    const { 1 },
"# , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..25#ROOT2024 1:Root[0000, 0]@0..25#ROOT2024
              IDENT   _ 1:Root[0000, 0]@5..6#ROOT2024
              PUNCH   ; [joint] 0:Root[0000, 0]@36..37#ROOT2024
              SUBTREE () 0:Root[0000, 0]@34..35#ROOT2024 0:Root[0000, 0]@34..35#ROOT2024
                IDENT   const 1:Root[0000, 0]@12..17#ROOT2024
                SUBTREE {} 1:Root[0000, 0]@18..19#ROOT2024 1:Root[0000, 0]@22..23#ROOT2024
                  LITERAL Integer 1 1:Root[0000, 0]@20..21#ROOT2024
              PUNCH   ; [alone] 0:Root[0000, 0]@39..40#ROOT2024

            _;
            (const  {
                1
            });"#]] ,) ; check (Edition :: Edition2021 , Edition :: Edition2024 , r#"
($($e:expr),* $(,)?) => {
    $($e);* ;
};
"# , r#"
    _,
"# , expect ! [[r#"
            ExpandError {
                inner: (
                    1:Root[0000, 0]@5..6#ROOT2024,
                    NoMatchingRule,
                ),
            }

            SUBTREE $$ 1:Root[0000, 0]@0..8#ROOT2024 1:Root[0000, 0]@0..8#ROOT2024
              PUNCH   ; [alone] 0:Root[0000, 0]@39..40#ROOT2024

            ;"#]] ,) ; check (Edition :: Edition2021 , Edition :: Edition2024 , r#"
($($e:expr),* $(,)?) => {
    $($e);* ;
};
"# , r#"
    const { 1 },
"# , expect ! [[r#"
            ExpandError {
                inner: (
                    1:Root[0000, 0]@5..10#ROOT2024,
                    NoMatchingRule,
                ),
            }

            SUBTREE $$ 1:Root[0000, 0]@0..18#ROOT2024 1:Root[0000, 0]@0..18#ROOT2024
              PUNCH   ; [alone] 0:Root[0000, 0]@39..40#ROOT2024

            ;"#]] ,) ; check (Edition :: Edition2024 , Edition :: Edition2024 , r#"
($($e:expr_2021),* $(,)?) => {
    $($e);* ;
};
"# , r#"
    4,
    "literal",
    funcall(),
    future.await,
    break 'foo bar,
"# , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..76#ROOT2024 1:Root[0000, 0]@0..76#ROOT2024
              LITERAL Integer 4 1:Root[0000, 0]@5..6#ROOT2024
              PUNCH   ; [joint] 0:Root[0000, 0]@41..42#ROOT2024
              LITERAL Str literal 1:Root[0000, 0]@12..21#ROOT2024
              PUNCH   ; [joint] 0:Root[0000, 0]@41..42#ROOT2024
              SUBTREE () 0:Root[0000, 0]@39..40#ROOT2024 0:Root[0000, 0]@39..40#ROOT2024
                IDENT   funcall 1:Root[0000, 0]@27..34#ROOT2024
                SUBTREE () 1:Root[0000, 0]@34..35#ROOT2024 1:Root[0000, 0]@35..36#ROOT2024
              PUNCH   ; [joint] 0:Root[0000, 0]@41..42#ROOT2024
              SUBTREE () 0:Root[0000, 0]@39..40#ROOT2024 0:Root[0000, 0]@39..40#ROOT2024
                IDENT   future 1:Root[0000, 0]@42..48#ROOT2024
                PUNCH   . [alone] 1:Root[0000, 0]@48..49#ROOT2024
                IDENT   await 1:Root[0000, 0]@49..54#ROOT2024
              PUNCH   ; [joint] 0:Root[0000, 0]@41..42#ROOT2024
              SUBTREE () 0:Root[0000, 0]@39..40#ROOT2024 0:Root[0000, 0]@39..40#ROOT2024
                IDENT   break 1:Root[0000, 0]@60..65#ROOT2024
                PUNCH   ' [joint] 1:Root[0000, 0]@66..67#ROOT2024
                IDENT   foo 1:Root[0000, 0]@67..70#ROOT2024
                IDENT   bar 1:Root[0000, 0]@71..74#ROOT2024
              PUNCH   ; [alone] 0:Root[0000, 0]@44..45#ROOT2024

            4;
            "literal";
            (funcall());
            (future.await);
            (break 'foo bar);"#]] ,) ; check (Edition :: Edition2024 , Edition :: Edition2024 , r#"
($($e:expr_2021),* $(,)?) => {
    $($e);* ;
};
"# , r#"
    _,
"# , expect ! [[r#"
            ExpandError {
                inner: (
                    1:Root[0000, 0]@5..6#ROOT2024,
                    NoMatchingRule,
                ),
            }

            SUBTREE $$ 1:Root[0000, 0]@0..8#ROOT2024 1:Root[0000, 0]@0..8#ROOT2024
              PUNCH   ; [alone] 0:Root[0000, 0]@44..45#ROOT2024

            ;"#]] ,) ; }
};
}
