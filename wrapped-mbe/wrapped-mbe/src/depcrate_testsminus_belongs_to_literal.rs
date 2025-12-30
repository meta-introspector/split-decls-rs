// Generated macro for minus_belongs_to_literal (function)
macro_rules! Depcrate_testsminus_belongs_to_literal {
() => {
// Module: crate::tests
// Provides: {"minus_belongs_to_literal"}
// Dependencies: {}
# [test] fn minus_belongs_to_literal () { let decl = r#"
(-1) => {-1};
(- 2) => {- 2};
(- 3.0) => {- 3.0};
(@$lit:literal) => {$lit}
"# ; let check = | args , expect | check (Edition :: CURRENT , Edition :: CURRENT , decl , args , expect) ; check ("-1" , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..2#ROOT2024 1:Root[0000, 0]@0..2#ROOT2024
              PUNCH   - [alone] 0:Root[0000, 0]@10..11#ROOT2024
              LITERAL Integer 1 0:Root[0000, 0]@11..12#ROOT2024

            -1"#]] ,) ; check ("- 1" , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..3#ROOT2024 1:Root[0000, 0]@0..3#ROOT2024
              PUNCH   - [alone] 0:Root[0000, 0]@10..11#ROOT2024
              LITERAL Integer 1 0:Root[0000, 0]@11..12#ROOT2024

            -1"#]] ,) ; check ("-2" , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..2#ROOT2024 1:Root[0000, 0]@0..2#ROOT2024
              PUNCH   - [alone] 0:Root[0000, 0]@25..26#ROOT2024
              LITERAL Integer 2 0:Root[0000, 0]@27..28#ROOT2024

            -2"#]] ,) ; check ("- 2" , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..3#ROOT2024 1:Root[0000, 0]@0..3#ROOT2024
              PUNCH   - [alone] 0:Root[0000, 0]@25..26#ROOT2024
              LITERAL Integer 2 0:Root[0000, 0]@27..28#ROOT2024

            -2"#]] ,) ; check ("-3.0" , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..4#ROOT2024 1:Root[0000, 0]@0..4#ROOT2024
              PUNCH   - [alone] 0:Root[0000, 0]@43..44#ROOT2024
              LITERAL Float 3.0 0:Root[0000, 0]@45..48#ROOT2024

            -3.0"#]] ,) ; check ("- 3.0" , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..5#ROOT2024 1:Root[0000, 0]@0..5#ROOT2024
              PUNCH   - [alone] 0:Root[0000, 0]@43..44#ROOT2024
              LITERAL Float 3.0 0:Root[0000, 0]@45..48#ROOT2024

            -3.0"#]] ,) ; check ("@1" , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..2#ROOT2024 1:Root[0000, 0]@0..2#ROOT2024
              LITERAL Integer 1 1:Root[0000, 0]@1..2#ROOT2024

            1"#]] ,) ; check ("@-1" , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..3#ROOT2024 1:Root[0000, 0]@0..3#ROOT2024
              PUNCH   - [alone] 1:Root[0000, 0]@1..2#ROOT2024
              LITERAL Integer 1 1:Root[0000, 0]@2..3#ROOT2024

            -1"#]] ,) ; check ("@1.0" , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..4#ROOT2024 1:Root[0000, 0]@0..4#ROOT2024
              LITERAL Float 1.0 1:Root[0000, 0]@1..4#ROOT2024

            1.0"#]] ,) ; check ("@-1.0" , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..5#ROOT2024 1:Root[0000, 0]@0..5#ROOT2024
              PUNCH   - [alone] 1:Root[0000, 0]@1..2#ROOT2024
              LITERAL Float 1.0 1:Root[0000, 0]@2..5#ROOT2024

            -1.0"#]] ,) ; check ("@--1.0" , expect ! [[r#"
            ExpandError {
                inner: (
                    1:Root[0000, 0]@1..2#ROOT2024,
                    BindingError(
                        "expected literal",
                    ),
                ),
            }

            SUBTREE $$ 1:Root[0000, 0]@0..6#ROOT2024 1:Root[0000, 0]@0..6#ROOT2024
              PUNCH   - [joint] 1:Root[0000, 0]@1..2#ROOT2024
              PUNCH   - [alone] 1:Root[0000, 0]@2..3#ROOT2024

            --"#]] ,) ; }
};
}
