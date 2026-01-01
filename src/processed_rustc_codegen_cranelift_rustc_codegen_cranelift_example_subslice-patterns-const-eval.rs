// SRC: ../rust/compiler/rustc_codegen_cranelift/example/subslice-patterns-const-eval.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=N(u8); | COMPLEXITY=8 | LINES=17 */
// Based on https://github.com/rust-lang/rust/blob/c5840f9d252c2f5cc16698dbf385a29c5de3ca07/src/test/ui/array-slice-vec/subslice-patterns-const-eval-match.rs

// Test that array subslice patterns are correctly handled in const evaluation.

// run-pass

#[derive(PartialEq, Debug, Clone)]
struct N(u8);

#[derive(PartialEq, Debug, Clone)]
struct Z;

macro_rules! n {
    ($($e:expr),* $(,)?) => {
        [$(N($e)),*]
    }
}
/* AST_META: AST_ID=2 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=8 */

// This macro has an unused variable so that it can be repeated base on the
// number of times a repeated variable (`$e` in `z`) occurs.
macro_rules! zed {
    ($e:expr) => {
        Z
    };
}
/* AST_META: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=6 */

macro_rules! z {
    ($($e:expr),* $(,)?) => {
        [$(zed!($e)),*]
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=10 | LINES=14 */

// Compare constant evaluation and runtime evaluation of a given expression.
macro_rules! compare_evaluation {
    ($e:expr, $t:ty $(,)?) => {{
        const CONST_EVAL: $t = $e;
        const fn const_eval() -> $t {
            $e
        }
        static CONST_EVAL2: $t = const_eval();
        let runtime_eval = $e;
        assert_eq!(CONST_EVAL, runtime_eval);
        assert_eq!(CONST_EVAL2, runtime_eval);
    }};
}
/* AST_META: AST_ID=5 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

// Repeat `$test`, substituting the given macro variables with the given
// identifiers.
//
// For example:
//
// repeat! {
//     ($name); X; Y:
//     struct $name;
// }
/* AST_META: AST_ID=6 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=15 | LINES=15 */
//
// Expands to:
//
// struct X; struct Y;
//
// This is used to repeat the tests using both the `N` and `Z`
// types.
macro_rules! repeat {
    (($($dollar:tt $placeholder:ident)*); $($($values:ident),+);*: $($test:tt)*) => {
        macro_rules! single {
            ($($dollar $placeholder:ident),*) => { $($test)* }
        }
        $(single!($($values),+);)*
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=main | COMPLEXITY=20 | LINES=32 */

#[rustfmt::skip]
fn main() {
    repeat! {
        ($arr $Ty); n, N; z, Z:
        compare_evaluation!({ let [_, x @ .., _] = $arr!(1, 2, 3, 4); x }, [$Ty; 2]);
        compare_evaluation!({ let [_, ref x @ .., _] = $arr!(1, 2, 3, 4); x }, &'static [$Ty; 2]);
        compare_evaluation!({ let [_, x @ .., _] = &$arr!(1, 2, 3, 4); x }, &'static [$Ty; 2]);

        compare_evaluation!({ let [_, _, x @ .., _, _] = $arr!(1, 2, 3, 4); x }, [$Ty; 0]);
        compare_evaluation!(
            { let [_, _, ref x @ .., _, _] = $arr!(1, 2, 3, 4); x },
            &'static [$Ty; 0],
        );
        compare_evaluation!(
            { let [_, _, x @ .., _, _] = &$arr!(1, 2, 3, 4); x },
            &'static [$Ty; 0],
        );

        compare_evaluation!({ let [_, .., x] = $arr!(1, 2, 3, 4); x }, $Ty);
        compare_evaluation!({ let [_, .., ref x] = $arr!(1, 2, 3, 4); x }, &'static $Ty);
        compare_evaluation!({ let [_, _y @ .., x] = &$arr!(1, 2, 3, 4); x }, &'static $Ty);
    }

    compare_evaluation!({ let [_, .., N(x)] = n!(1, 2, 3, 4); x }, u8);
    compare_evaluation!({ let [_, .., N(ref x)] = n!(1, 2, 3, 4); x }, &'static u8);
    compare_evaluation!({ let [_, .., N(x)] = &n!(1, 2, 3, 4); x }, &'static u8);

    compare_evaluation!({ let [N(x), .., _] = n!(1, 2, 3, 4); x }, u8);
    compare_evaluation!({ let [N(ref x), .., _] = n!(1, 2, 3, 4); x }, &'static u8);
    compare_evaluation!({ let [N(x), .., _] = &n!(1, 2, 3, 4); x }, &'static u8);
}