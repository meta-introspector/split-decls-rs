// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_fragments_on_composite_typestests {
() => {
// Module: crate::validation::rules::fragments_on_composite_types
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn on_object () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment validFragment on Dog {
            barks
          }
        "# ,) ; } # [test] fn on_interface () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment validFragment on Pet {
            name
          }
        "# ,) ; } # [test] fn on_object_inline () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment validFragment on Pet {
            ... on Dog {
              barks
            }
          }
        "# ,) ; } # [test] fn on_inline_without_type_cond () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment validFragment on Pet {
            ... {
              name
            }
          }
        "# ,) ; } # [test] fn on_union () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment validFragment on CatOrDog {
            __typename
          }
        "# ,) ; } # [test] fn not_on_scalar () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment scalarFragment on Boolean {
            bad
          }
        "# , & [RuleError :: new (& error_message (Some ("scalarFragment") , "Boolean") , & [SourcePosition :: new (38 , 1 , 37)] ,)] ,) ; } # [test] fn not_on_enum () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment scalarFragment on FurColor {
            bad
          }
        "# , & [RuleError :: new (& error_message (Some ("scalarFragment") , "FurColor") , & [SourcePosition :: new (38 , 1 , 37)] ,)] ,) ; } # [test] fn not_on_input_object () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment inputFragment on ComplexInput {
            stringField
          }
        "# , & [RuleError :: new (& error_message (Some ("inputFragment") , "ComplexInput") , & [SourcePosition :: new (37 , 1 , 36)] ,)] ,) ; } # [test] fn not_on_scalar_inline () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment invalidFragment on Pet {
            ... on String {
              barks
            }
          }
        "# , & [RuleError :: new (& error_message (None , "String") , & [SourcePosition :: new (64 , 2 , 19)] ,)] ,) ; } }
};
}
