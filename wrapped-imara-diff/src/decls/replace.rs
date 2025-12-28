macro_rules! deps {
    () => {
        Algorithm!();
        BasicLineDiffPrinter!();
        Diff!();
        InternedInput!();
        UnifiedDiffConfig!();
    };
}

macro_rules! replace {
    () => {
        deps!();
        # [test] fn replace () { let before = r#"fn foo() -> Bar{
    let mut foo = 2.0;
    foo *= 100 / 2;
    println!("hello world")        
}
"# ; let after = r#"const TEST: i32 = 0;
fn foo() -> Bar{
    let mut foo = 2.0;
    foo *= 100 / 2;
    println!("hello world");        
    println!("hello foo {TEST}");        
}
    
"# ; let input = InternedInput :: new (before , after) ; for algorithm in Algorithm :: ALL { println ! ("{algorithm:?}") ; let mut diff = Diff :: compute (algorithm , & input) ; diff . postprocess_lines (& input) ; expect ! [[r#"
            @@ -1,5 +1,8 @@
            +const TEST: i32 = 0;
             fn foo() -> Bar{
                 let mut foo = 2.0;
                 foo *= 100 / 2;
            -    println!("hello world")        
            +    println!("hello world");        
            +    println!("hello foo {TEST}");        
             }
            +    
        "#]] . assert_eq (& diff . unified_diff (& BasicLineDiffPrinter (& input . interner) , UnifiedDiffConfig :: default () , & input ,) . to_string () ,) ; } }
    };
}

replace!()