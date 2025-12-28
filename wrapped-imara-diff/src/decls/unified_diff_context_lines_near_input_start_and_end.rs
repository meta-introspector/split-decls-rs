macro_rules! deps {
    () => {
        BasicLineDiffPrinter!();
        UnifiedDiffConfig!();
        Diff!();
        InternedInput!();
        Algorithm!();
    };
}

macro_rules! unified_diff_context_lines_near_input_start_and_end {
    () => {
        deps!();
        # [test] fn unified_diff_context_lines_near_input_start_and_end () { let before = r#"a
b
c
d
e
f
g
h
i
"# ; let after = r#"a
b
c
d
edit
f
g
h
i
"# ; let input = InternedInput :: new (before , after) ; for algorithm in Algorithm :: ALL { println ! ("{algorithm:?}") ; let mut diff = Diff :: compute (algorithm , & input) ; diff . postprocess_lines (& input) ; expect ! [[r#"
          @@ -2,7 +2,7 @@
           b
           c
           d
          -e
          +edit
           f
           g
           h
          "#]] . assert_eq (& diff . unified_diff (& BasicLineDiffPrinter (& input . interner) , UnifiedDiffConfig :: default () , & input ,) . to_string () ,) ; } }
    };
}

unified_diff_context_lines_near_input_start_and_end!();