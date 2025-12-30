// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_no_unused_variablestests {
() => {
// Module: crate::validation::rules::no_unused_variables
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn uses_all_variables () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query ($a: String, $b: String, $c: String) {
            field(a: $a, b: $b, c: $c)
          }
        "# ,) ; } # [test] fn uses_all_variables_deeply () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String, $b: String, $c: String) {
            field(a: $a) {
              field(b: $b) {
                field(c: $c)
              }
            }
          }
        "# ,) ; } # [test] fn uses_all_variables_deeply_in_inline_fragments () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String, $b: String, $c: String) {
            ... on Type {
              field(a: $a) {
                field(b: $b) {
                  ... on Type {
                    field(c: $c)
                  }
                }
              }
            }
          }
        "# ,) ; } # [test] fn uses_all_variables_in_fragments () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String, $b: String, $c: String) {
            ...FragA
          }
          fragment FragA on Type {
            field(a: $a) {
              ...FragB
            }
          }
          fragment FragB on Type {
            field(b: $b) {
              ...FragC
            }
          }
          fragment FragC on Type {
            field(c: $c)
          }
        "# ,) ; } # [test] fn variable_used_by_fragment_in_multiple_operations () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String) {
            ...FragA
          }
          query Bar($b: String) {
            ...FragB
          }
          fragment FragA on Type {
            field(a: $a)
          }
          fragment FragB on Type {
            field(b: $b)
          }
        "# ,) ; } # [test] fn variable_used_by_recursive_fragment () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String) {
            ...FragA
          }
          fragment FragA on Type {
            field(a: $a) {
              ...FragA
            }
          }
        "# ,) ; } # [test] fn variable_not_used () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query ($a: String, $b: String, $c: String) {
            field(a: $a, b: $b)
          }
        "# , & [RuleError :: new (& error_message ("c" , None) , & [SourcePosition :: new (42 , 1 , 41)] ,)] ,) ; } # [test] fn multiple_variables_not_used_1 () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String, $b: String, $c: String) {
            field(b: $b)
          }
        "# , & [RuleError :: new (& error_message ("a" , Some ("Foo")) , & [SourcePosition :: new (21 , 1 , 20)] ,) , RuleError :: new (& error_message ("c" , Some ("Foo")) , & [SourcePosition :: new (45 , 1 , 44)] ,) ,] ,) ; } # [test] fn variable_not_used_in_fragment () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String, $b: String, $c: String) {
            ...FragA
          }
          fragment FragA on Type {
            field(a: $a) {
              ...FragB
            }
          }
          fragment FragB on Type {
            field(b: $b) {
              ...FragC
            }
          }
          fragment FragC on Type {
            field
          }
        "# , & [RuleError :: new (& error_message ("c" , Some ("Foo")) , & [SourcePosition :: new (45 , 1 , 44)] ,)] ,) ; } # [test] fn multiple_variables_not_used_2 () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String, $b: String, $c: String) {
            ...FragA
          }
          fragment FragA on Type {
            field {
              ...FragB
            }
          }
          fragment FragB on Type {
            field(b: $b) {
              ...FragC
            }
          }
          fragment FragC on Type {
            field
          }
        "# , & [RuleError :: new (& error_message ("a" , Some ("Foo")) , & [SourcePosition :: new (21 , 1 , 20)] ,) , RuleError :: new (& error_message ("c" , Some ("Foo")) , & [SourcePosition :: new (45 , 1 , 44)] ,) ,] ,) ; } # [test] fn variable_not_used_by_unreferenced_fragment () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($b: String) {
            ...FragA
          }
          fragment FragA on Type {
            field(a: $a)
          }
          fragment FragB on Type {
            field(b: $b)
          }
        "# , & [RuleError :: new (& error_message ("b" , Some ("Foo")) , & [SourcePosition :: new (21 , 1 , 20)] ,)] ,) ; } # [test] fn variable_not_used_by_fragment_used_by_other_operation () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($b: String) {
            ...FragA
          }
          query Bar($a: String) {
            ...FragB
          }
          fragment FragA on Type {
            field(a: $a)
          }
          fragment FragB on Type {
            field(b: $b)
          }
        "# , & [RuleError :: new (& error_message ("b" , Some ("Foo")) , & [SourcePosition :: new (21 , 1 , 20)] ,) , RuleError :: new (& error_message ("a" , Some ("Bar")) , & [SourcePosition :: new (88 , 4 , 20)] ,) ,] ,) ; } }
};
}
