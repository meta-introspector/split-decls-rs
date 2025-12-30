// Generated macro for simple_insert (function)
macro_rules! Depcrate_testssimple_insert {
() => {
// Module: crate::tests
// Provides: {"simple_insert"}
// Dependencies: {}
# [test] fn simple_insert () { let before = r#"fn foo() -> Bar{
    let mut foo = 2.0;
    foo *= 100 / 2;
}"# ; let after = r#"fn foo() -> Bar{
    let mut foo = 2.0;
    foo *= 100 / 2;
    println("hello world")
}"# ; let mut input = InternedInput :: new (before , after) ; for algorithm in Algorithm :: ALL { println ! ("{algorithm:?}") ; let mut diff = Diff :: compute (algorithm , & input) ; diff . postprocess_lines (& input) ; expect ! [[r#"
          @@ -1,4 +1,5 @@
           fn foo() -> Bar{
               let mut foo = 2.0;
               foo *= 100 / 2;
          +    println("hello world")
           }
          "#]] . assert_eq (& diff . unified_diff (& BasicLineDiffPrinter (& input . interner) , UnifiedDiffConfig :: default () , & input ,) . to_string () ,) ; swap (& mut input . before , & mut input . after) ; let mut diff = Diff :: compute (algorithm , & input) ; diff . postprocess_lines (& input) ; expect ! [[r#"
            @@ -1,5 +1,4 @@
             fn foo() -> Bar{
                 let mut foo = 2.0;
                 foo *= 100 / 2;
            -    println("hello world")
             }
            "#]] . assert_eq (& diff . unified_diff (& BasicLineDiffPrinter (& input . interner) , UnifiedDiffConfig :: default () , & input ,) . to_string () ,) ; swap (& mut input . before , & mut input . after) ; } }
};
}
