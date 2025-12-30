// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_known_argument_namestests {
() => {
// Module: crate::validation::rules::known_argument_names
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { directive_error_message , factory , field_error_message } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn single_arg_is_known () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment argOnRequiredArg on Dog {
            doesKnowCommand(dogCommand: SIT)
          }
        "# ,) ; } # [test] fn multiple_args_are_known () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment multipleArgs on ComplicatedArgs {
            multipleReqs(req1: 1, req2: 2)
          }
        "# ,) ; } # [test] fn ignores_args_of_unknown_fields () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment argOnUnknownField on Dog {
            unknownField(unknownArg: SIT)
          }
        "# ,) ; } # [test] fn multiple_args_in_reverse_order_are_known () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment multipleArgsReverseOrder on ComplicatedArgs {
            multipleReqs(req2: 2, req1: 1)
          }
        "# ,) ; } # [test] fn no_args_on_optional_arg () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment noArgOnOptionalArg on Dog {
            isHousetrained
          }
        "# ,) ; } # [test] fn args_are_known_deeply () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog {
              doesKnowCommand(dogCommand: SIT)
            }
            human {
              pet {
                ... on Dog {
                  doesKnowCommand(dogCommand: SIT)
                }
              }
            }
          }
        "# ,) ; } # [test] fn directive_args_are_known () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog @skip(if: true)
          }
        "# ,) ; } # [test] fn undirective_args_are_invalid () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog @skip(unless: true)
          }
        "# , & [RuleError :: new (& directive_error_message ("unless" , "skip") , & [SourcePosition :: new (35 , 2 , 22)] ,)] ,) ; } # [test] fn invalid_arg_name () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment invalidArgName on Dog {
            doesKnowCommand(unknown: true)
          }
        "# , & [RuleError :: new (& field_error_message ("unknown" , "doesKnowCommand" , "Dog") , & [SourcePosition :: new (72 , 2 , 28)] ,)] ,) ; } # [test] fn unknown_args_amongst_known_args () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment oneGoodArgOneInvalidArg on Dog {
            doesKnowCommand(whoknows: 1, dogCommand: SIT, unknown: true)
          }
        "# , & [RuleError :: new (& field_error_message ("whoknows" , "doesKnowCommand" , "Dog") , & [SourcePosition :: new (81 , 2 , 28)] ,) , RuleError :: new (& field_error_message ("unknown" , "doesKnowCommand" , "Dog") , & [SourcePosition :: new (111 , 2 , 58)] ,) ,] ,) ; } # [test] fn unknown_args_deeply () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog {
              doesKnowCommand(unknown: true)
            }
            human {
              pet {
                ... on Dog {
                  doesKnowCommand(unknown: true)
                }
              }
            }
          }
        "# , & [RuleError :: new (& field_error_message ("unknown" , "doesKnowCommand" , "Dog") , & [SourcePosition :: new (61 , 3 , 30)] ,) , RuleError :: new (& field_error_message ("unknown" , "doesKnowCommand" , "Dog") , & [SourcePosition :: new (193 , 8 , 34)] ,) ,] ,) ; } }
};
}
