// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_unique_operation_namestests {
() => {
// Module: crate::validation::rules::unique_operation_names
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn no_operations () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Dog {
            name
          }
        "# ,) ; } # [test] fn one_anon_operation () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            field
          }
        "# ,) ; } # [test] fn one_named_operation () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            field
          }
        "# ,) ; } # [test] fn multiple_operations () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            dog {
              name
            }
          }

          query Bar {
            dog {
              name
            }
          }
        "# ,) ; } # [test] fn multiple_operations_of_different_types () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            field
          }

          mutation Bar {
            field
          }
        "# ,) ; } # [test] fn fragment_and_operation_named_the_same () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            dog {
              ...Foo
            }
          }
          fragment Foo on Dog {
            name
          }
        "# ,) ; } # [test] fn multiple_operations_of_same_name () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            dog {
              name
            }
          }
          query Foo {
            human {
              name
            }
          }
        "# , & [RuleError :: new (& error_message ("Foo") , & [SourcePosition :: new (11 , 1 , 10) , SourcePosition :: new (96 , 6 , 10) ,] ,)] ,) ; } # [test] fn multiple_ops_of_same_name_of_different_types () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            dog {
              name
            }
          }
          mutation Foo {
            testInput
          }
        "# , & [RuleError :: new (& error_message ("Foo") , & [SourcePosition :: new (11 , 1 , 10) , SourcePosition :: new (96 , 6 , 10) ,] ,)] ,) ; } }
};
}
