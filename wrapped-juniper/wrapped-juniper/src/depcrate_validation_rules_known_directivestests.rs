// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_known_directivestests {
() => {
// Module: crate::validation::rules::known_directives
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { factory , misplaced_error_message , unknown_error_message } ; use crate :: { parser :: SourcePosition , schema :: model :: DirectiveLocation , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn with_no_directives () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            name
            ...Frag
          }

          fragment Frag on Dog {
            name
          }
        "# ,) ; } # [test] fn with_known_directives () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog @include(if: true) {
              name
            }
            human @skip(if: false) {
              name
            }
          }
        "# ,) ; } # [test] fn with_unknown_directive () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog @unknown(directive: "value") {
              name
            }
          }
        "# , & [RuleError :: new (& unknown_error_message ("unknown") , & [SourcePosition :: new (29 , 2 , 16)] ,)] ,) ; } # [test] fn with_unknown_directive_on_var_definition () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"query Foo(
                $var1: Int = 1 @skip(if: true) @unknown,
                $var2: String @deprecated
            ) {
                name
            }"# , & [RuleError :: new (& misplaced_error_message ("skip" , & DirectiveLocation :: VariableDefinition) , & [SourcePosition :: new (42 , 1 , 31)] ,) , RuleError :: new (& unknown_error_message ("unknown") , & [SourcePosition :: new (58 , 1 , 47)] ,) , RuleError :: new (& misplaced_error_message ("deprecated" , & DirectiveLocation :: VariableDefinition) , & [SourcePosition :: new (98 , 2 , 30)] ,) ,] ,) ; } # [test] fn with_many_unknown_directives () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog @unknown(directive: "value") {
              name
            }
            human @unknown(directive: "value") {
              name
              pets @unknown(directive: "value") {
                name
              }
            }
          }
        "# , & [RuleError :: new (& unknown_error_message ("unknown") , & [SourcePosition :: new (29 , 2 , 16)] ,) , RuleError :: new (& unknown_error_message ("unknown") , & [SourcePosition :: new (111 , 5 , 18)] ,) , RuleError :: new (& unknown_error_message ("unknown") , & [SourcePosition :: new (180 , 7 , 19)] ,) ,] ,) ; } # [test] fn with_well_placed_directives () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo @onQuery {
            name @include(if: true)
            ...Frag @include(if: true)
            skippedField @skip(if: true)
            ...SkippedFrag @skip(if: true)
          }

          mutation Bar @onMutation {
            someField
          }
        "# ,) ; } # [test] fn with_misplaced_directives () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo @include(if: true) {
            name @onQuery
            ...Frag @onQuery
          }

          mutation Bar @onQuery {
            someField
          }
        "# , & [RuleError :: new (& misplaced_error_message ("include" , & DirectiveLocation :: Query) , & [SourcePosition :: new (21 , 1 , 20)] ,) , RuleError :: new (& misplaced_error_message ("onQuery" , & DirectiveLocation :: Field) , & [SourcePosition :: new (59 , 2 , 17)] ,) , RuleError :: new (& misplaced_error_message ("onQuery" , & DirectiveLocation :: FragmentSpread) , & [SourcePosition :: new (88 , 3 , 20)] ,) , RuleError :: new (& misplaced_error_message ("onQuery" , & DirectiveLocation :: Mutation) , & [SourcePosition :: new (133 , 6 , 23)] ,) ,] ,) ; } }
};
}
