// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_lone_anonymous_operationtests {
() => {
// Module: crate::validation::rules::lone_anonymous_operation
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn no_operations () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Type {
            field
          }
        "# ,) ; } # [test] fn one_anon_operation () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            field
          }
        "# ,) ; } # [test] fn multiple_named_operations () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            field
          }

          query Bar {
            field
          }
        "# ,) ; } # [test] fn anon_operation_with_fragment () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            ...Foo
          }
          fragment Foo on Type {
            field
          }
        "# ,) ; } # [test] fn multiple_anon_operations () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            fieldA
          }
          {
            fieldB
          }
        "# , & [RuleError :: new (error_message () , & [SourcePosition :: new (11 , 1 , 10)]) , RuleError :: new (error_message () , & [SourcePosition :: new (54 , 4 , 10)]) ,] ,) ; } # [test] fn anon_operation_with_a_mutation () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            fieldA
          }
          mutation Foo {
            fieldB
          }
        "# , & [RuleError :: new (error_message () , & [SourcePosition :: new (11 , 1 , 10)] ,)] ,) ; } }
};
}
