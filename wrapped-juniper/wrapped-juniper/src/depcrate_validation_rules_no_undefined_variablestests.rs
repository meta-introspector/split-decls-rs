// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_no_undefined_variablestests {
() => {
// Module: crate::validation::rules::no_undefined_variables
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn all_variables_defined () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String, $b: String, $c: String) {
            field(a: $a, b: $b, c: $c)
          }
        "# ,) ; } # [test] fn all_variables_deeply_defined () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String, $b: String, $c: String) {
            field(a: $a) {
              field(b: $b) {
                field(c: $c)
              }
            }
          }
        "# ,) ; } # [test] fn all_variables_deeply_defined_in_inline_fragments_defined () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
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
        "# ,) ; } # [test] fn all_variables_in_fragments_deeply_defined () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
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
        "# ,) ; } # [test] fn variable_within_single_fragment_defined_in_multiple_operations () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String) {
            ...FragA
          }
          query Bar($a: String) {
            ...FragA
          }
          fragment FragA on Type {
            field(a: $a)
          }
        "# ,) ; } # [test] fn variable_within_fragments_defined_in_operations () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
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
        "# ,) ; } # [test] fn variable_within_recursive_fragment_defined () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String) {
            ...FragA
          }
          fragment FragA on Type {
            field(a: $a) {
              ...FragA
            }
          }
        "# ,) ; } # [test] fn variable_not_defined () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String, $b: String, $c: String) {
            field(a: $a, b: $b, c: $c, d: $d)
          }
        "# , & [RuleError :: new (& error_message ("d" , Some ("Foo")) , & [SourcePosition :: new (101 , 2 , 42) , SourcePosition :: new (11 , 1 , 10) ,] ,)] ,) ; } # [test] fn variable_not_defined_by_unnamed_query () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            field(a: $a)
          }
        "# , & [RuleError :: new (& error_message ("a" , None) , & [SourcePosition :: new (34 , 2 , 21) , SourcePosition :: new (11 , 1 , 10) ,] ,)] ,) ; } # [test] fn multiple_variables_not_defined () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($b: String) {
            field(a: $a, b: $b, c: $c)
          }
        "# , & [RuleError :: new (& error_message ("a" , Some ("Foo")) , & [SourcePosition :: new (56 , 2 , 21) , SourcePosition :: new (11 , 1 , 10) ,] ,) , RuleError :: new (& error_message ("c" , Some ("Foo")) , & [SourcePosition :: new (70 , 2 , 35) , SourcePosition :: new (11 , 1 , 10) ,] ,) ,] ,) ; } # [test] fn variable_in_fragment_not_defined_by_unnamed_query () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            ...FragA
          }
          fragment FragA on Type {
            field(a: $a)
          }
        "# , & [RuleError :: new (& error_message ("a" , None) , & [SourcePosition :: new (102 , 5 , 21) , SourcePosition :: new (11 , 1 , 10) ,] ,)] ,) ; } # [test] fn variable_in_fragment_not_defined_by_operation () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String, $b: String) {
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
        "# , & [RuleError :: new (& error_message ("c" , Some ("Foo")) , & [SourcePosition :: new (358 , 15 , 21) , SourcePosition :: new (11 , 1 , 10) ,] ,)] ,) ; } # [test] fn multiple_variables_in_fragments_not_defined () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($b: String) {
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
        "# , & [RuleError :: new (& error_message ("a" , Some ("Foo")) , & [SourcePosition :: new (124 , 5 , 21) , SourcePosition :: new (11 , 1 , 10) ,] ,) , RuleError :: new (& error_message ("c" , Some ("Foo")) , & [SourcePosition :: new (346 , 15 , 21) , SourcePosition :: new (11 , 1 , 10) ,] ,) ,] ,) ; } # [test] fn single_variable_in_fragment_not_defined_by_multiple_operations () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String) {
            ...FragAB
          }
          query Bar($a: String) {
            ...FragAB
          }
          fragment FragAB on Type {
            field(a: $a, b: $b)
          }
        "# , & [RuleError :: new (& error_message ("b" , Some ("Foo")) , & [SourcePosition :: new (201 , 8 , 28) , SourcePosition :: new (11 , 1 , 10) ,] ,) , RuleError :: new (& error_message ("b" , Some ("Bar")) , & [SourcePosition :: new (201 , 8 , 28) , SourcePosition :: new (79 , 4 , 10) ,] ,) ,] ,) ; } # [test] fn variables_in_fragment_not_defined_by_multiple_operations () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($b: String) {
            ...FragAB
          }
          query Bar($a: String) {
            ...FragAB
          }
          fragment FragAB on Type {
            field(a: $a, b: $b)
          }
        "# , & [RuleError :: new (& error_message ("a" , Some ("Foo")) , & [SourcePosition :: new (194 , 8 , 21) , SourcePosition :: new (11 , 1 , 10) ,] ,) , RuleError :: new (& error_message ("b" , Some ("Bar")) , & [SourcePosition :: new (201 , 8 , 28) , SourcePosition :: new (79 , 4 , 10) ,] ,) ,] ,) ; } # [test] fn variable_in_fragment_used_by_other_operation () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
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
        "# , & [RuleError :: new (& error_message ("a" , Some ("Foo")) , & [SourcePosition :: new (191 , 8 , 21) , SourcePosition :: new (11 , 1 , 10) ,] ,) , RuleError :: new (& error_message ("b" , Some ("Bar")) , & [SourcePosition :: new (263 , 11 , 21) , SourcePosition :: new (78 , 4 , 10) ,] ,) ,] ,) ; } # [test] fn multiple_undefined_variables_produce_multiple_errors () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($b: String) {
            ...FragAB
          }
          query Bar($a: String) {
            ...FragAB
          }
          fragment FragAB on Type {
            field1(a: $a, b: $b)
            ...FragC
            field3(a: $a, b: $b)
          }
          fragment FragC on Type {
            field2(c: $c)
          }
        "# , & [RuleError :: new (& error_message ("a" , Some ("Foo")) , & [SourcePosition :: new (195 , 8 , 22) , SourcePosition :: new (11 , 1 , 10) ,] ,) , RuleError :: new (& error_message ("b" , Some ("Bar")) , & [SourcePosition :: new (202 , 8 , 29) , SourcePosition :: new (79 , 4 , 10) ,] ,) , RuleError :: new (& error_message ("a" , Some ("Foo")) , & [SourcePosition :: new (249 , 10 , 22) , SourcePosition :: new (11 , 1 , 10) ,] ,) , RuleError :: new (& error_message ("b" , Some ("Bar")) , & [SourcePosition :: new (256 , 10 , 29) , SourcePosition :: new (79 , 4 , 10) ,] ,) , RuleError :: new (& error_message ("c" , Some ("Foo")) , & [SourcePosition :: new (329 , 13 , 22) , SourcePosition :: new (11 , 1 , 10) ,] ,) , RuleError :: new (& error_message ("c" , Some ("Bar")) , & [SourcePosition :: new (329 , 13 , 22) , SourcePosition :: new (79 , 4 , 10) ,] ,) ,] ,) ; } }
};
}
