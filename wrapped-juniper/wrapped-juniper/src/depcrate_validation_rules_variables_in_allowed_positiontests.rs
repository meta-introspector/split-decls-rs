// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_variables_in_allowed_positiontests {
() => {
// Module: crate::validation::rules::variables_in_allowed_position
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn boolean_into_boolean () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($booleanArg: Boolean)
          {
            complicatedArgs {
              booleanArgField(booleanArg: $booleanArg)
            }
          }
        "# ,) ; } # [test] fn boolean_into_boolean_within_fragment () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment booleanArgFrag on ComplicatedArgs {
            booleanArgField(booleanArg: $booleanArg)
          }
          query Query($booleanArg: Boolean)
          {
            complicatedArgs {
              ...booleanArgFrag
            }
          }
        "# ,) ; expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($booleanArg: Boolean)
          {
            complicatedArgs {
              ...booleanArgFrag
            }
          }
          fragment booleanArgFrag on ComplicatedArgs {
            booleanArgField(booleanArg: $booleanArg)
          }
        "# ,) ; } # [test] fn non_null_boolean_into_boolean () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($nonNullBooleanArg: Boolean!)
          {
            complicatedArgs {
              booleanArgField(booleanArg: $nonNullBooleanArg)
            }
          }
        "# ,) ; } # [test] fn non_null_boolean_into_boolean_within_fragment () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment booleanArgFrag on ComplicatedArgs {
            booleanArgField(booleanArg: $nonNullBooleanArg)
          }

          query Query($nonNullBooleanArg: Boolean!)
          {
            complicatedArgs {
              ...booleanArgFrag
            }
          }
        "# ,) ; } # [test] fn int_into_non_null_int_with_default () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($intArg: Int = 1)
          {
            complicatedArgs {
              nonNullIntArgField(nonNullIntArg: $intArg)
            }
          }
        "# ,) ; } # [test] fn string_list_into_string_list () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($stringListVar: [String])
          {
            complicatedArgs {
              stringListArgField(stringListArg: $stringListVar)
            }
          }
        "# ,) ; } # [test] fn non_null_string_list_into_string_list () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($stringListVar: [String!])
          {
            complicatedArgs {
              stringListArgField(stringListArg: $stringListVar)
            }
          }
        "# ,) ; } # [test] fn string_into_string_list_in_item_position () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($stringVar: String)
          {
            complicatedArgs {
              stringListArgField(stringListArg: [$stringVar])
            }
          }
        "# ,) ; } # [test] fn non_null_string_into_string_list_in_item_position () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($stringVar: String!)
          {
            complicatedArgs {
              stringListArgField(stringListArg: [$stringVar])
            }
          }
        "# ,) ; } # [test] fn complex_input_into_complex_input () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($complexVar: ComplexInput)
          {
            complicatedArgs {
              complexArgField(complexArg: $complexVar)
            }
          }
        "# ,) ; } # [test] fn complex_input_into_complex_input_in_field_position () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($boolVar: Boolean = false)
          {
            complicatedArgs {
              complexArgField(complexArg: {requiredArg: $boolVar})
            }
          }
        "# ,) ; } # [test] fn non_null_boolean_into_non_null_boolean_in_directive () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($boolVar: Boolean!)
          {
            dog @include(if: $boolVar)
          }
        "# ,) ; } # [test] fn boolean_in_non_null_in_directive_with_default () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($boolVar: Boolean = false)
          {
            dog @include(if: $boolVar)
          }
        "# ,) ; } # [test] fn int_into_non_null_int () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($intArg: Int) {
            complicatedArgs {
              nonNullIntArgField(nonNullIntArg: $intArg)
            }
          }
        "# , & [RuleError :: new (& error_message ("intArg" , "Int" , "Int!") , & [SourcePosition :: new (23 , 1 , 22) , SourcePosition :: new (117 , 3 , 48) ,] ,)] ,) ; } # [test] fn int_into_non_null_int_within_fragment () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment nonNullIntArgFieldFrag on ComplicatedArgs {
            nonNullIntArgField(nonNullIntArg: $intArg)
          }

          query Query($intArg: Int) {
            complicatedArgs {
              ...nonNullIntArgFieldFrag
            }
          }
        "# , & [RuleError :: new (& error_message ("intArg" , "Int" , "Int!") , & [SourcePosition :: new (154 , 5 , 22) , SourcePosition :: new (110 , 2 , 46) ,] ,)] ,) ; } # [test] fn int_into_non_null_int_within_nested_fragment () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment outerFrag on ComplicatedArgs {
            ...nonNullIntArgFieldFrag
          }

          fragment nonNullIntArgFieldFrag on ComplicatedArgs {
            nonNullIntArgField(nonNullIntArg: $intArg)
          }

          query Query($intArg: Int) {
            complicatedArgs {
              ...outerFrag
            }
          }
        "# , & [RuleError :: new (& error_message ("intArg" , "Int" , "Int!") , & [SourcePosition :: new (255 , 9 , 22) , SourcePosition :: new (211 , 6 , 46) ,] ,)] ,) ; } # [test] fn string_over_boolean () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($stringVar: String) {
            complicatedArgs {
              booleanArgField(booleanArg: $stringVar)
            }
          }
        "# , & [RuleError :: new (& error_message ("stringVar" , "String" , "Boolean") , & [SourcePosition :: new (23 , 1 , 22) , SourcePosition :: new (117 , 3 , 42) ,] ,)] ,) ; } # [test] fn string_into_string_list () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($stringVar: String) {
            complicatedArgs {
              stringListArgField(stringListArg: $stringVar)
            }
          }
        "# , & [RuleError :: new (& error_message ("stringVar" , "String" , "[String]") , & [SourcePosition :: new (23 , 1 , 22) , SourcePosition :: new (123 , 3 , 48) ,] ,)] ,) ; } # [test] fn boolean_into_non_null_boolean_in_directive () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($boolVar: Boolean) {
            dog @include(if: $boolVar)
          }
        "# , & [RuleError :: new (& error_message ("boolVar" , "Boolean" , "Boolean!") , & [SourcePosition :: new (23 , 1 , 22) , SourcePosition :: new (73 , 2 , 29) ,] ,)] ,) ; } # [test] fn string_into_non_null_boolean_in_directive () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Query($stringVar: String) {
            dog @include(if: $stringVar)
          }
        "# , & [RuleError :: new (& error_message ("stringVar" , "String" , "Boolean!") , & [SourcePosition :: new (23 , 1 , 22) , SourcePosition :: new (74 , 2 , 29) ,] ,)] ,) ; } }
};
}
