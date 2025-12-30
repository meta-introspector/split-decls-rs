// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_scalar_leafstests {
() => {
// Module: crate::validation::rules::scalar_leafs
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { factory , no_allowed_error_message , required_error_message } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn valid_scalar_selection () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment scalarSelection on Dog {
            barks
          }
        "# ,) ; } # [test] fn object_type_missing_selection () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query directQueryOnObjectWithoutSubFields {
            human
          }
        "# , & [RuleError :: new (& required_error_message ("human" , "Human") , & [SourcePosition :: new (67 , 2 , 12)] ,)] ,) ; } # [test] fn interface_type_missing_selection () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            human { pets }
          }
        "# , & [RuleError :: new (& required_error_message ("pets" , "[Pet]") , & [SourcePosition :: new (33 , 2 , 20)] ,)] ,) ; } # [test] fn valid_scalar_selection_with_args () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment scalarSelectionWithArgs on Dog {
            doesKnowCommand(dogCommand: SIT)
          }
        "# ,) ; } # [test] fn scalar_selection_not_allowed_on_boolean () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment scalarSelectionsNotAllowedOnBoolean on Dog {
            barks { sinceWhen }
          }
        "# , & [RuleError :: new (& no_allowed_error_message ("barks" , "Boolean") , & [SourcePosition :: new (77 , 2 , 12)] ,)] ,) ; } # [test] fn scalar_selection_not_allowed_on_enum () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment scalarSelectionsNotAllowedOnEnum on Cat {
            furColor { inHexdec }
          }
        "# , & [RuleError :: new (& no_allowed_error_message ("furColor" , "FurColor") , & [SourcePosition :: new (74 , 2 , 12)] ,)] ,) ; } # [test] fn scalar_selection_not_allowed_with_args () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment scalarSelectionsNotAllowedWithArgs on Dog {
            doesKnowCommand(dogCommand: SIT) { sinceWhen }
          }
        "# , & [RuleError :: new (& no_allowed_error_message ("doesKnowCommand" , "Boolean") , & [SourcePosition :: new (76 , 2 , 12)] ,)] ,) ; } # [test] fn scalar_selection_not_allowed_with_directives () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment scalarSelectionsNotAllowedWithDirectives on Dog {
            name @include(if: true) { isAlsoHumanName }
          }
        "# , & [RuleError :: new (& no_allowed_error_message ("name" , "String") , & [SourcePosition :: new (82 , 2 , 12)] ,)] ,) ; } # [test] fn scalar_selection_not_allowed_with_directives_and_args () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment scalarSelectionsNotAllowedWithDirectivesAndArgs on Dog {
            doesKnowCommand(dogCommand: SIT) @include(if: true) { sinceWhen }
          }
        "# , & [RuleError :: new (& no_allowed_error_message ("doesKnowCommand" , "Boolean") , & [SourcePosition :: new (89 , 2 , 12)] ,)] ,) ; } }
};
}
