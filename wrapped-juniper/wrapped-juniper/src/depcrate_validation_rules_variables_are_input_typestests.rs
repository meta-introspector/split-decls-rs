// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_variables_are_input_typestests {
() => {
// Module: crate::validation::rules::variables_are_input_types
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn input_types_are_valid () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: String, $b: [Boolean!]!, $c: ComplexInput) {
            field(a: $a, b: $b, c: $c)
          }
        "# ,) ; } # [test] fn output_types_are_invalid () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($a: Dog, $b: [[CatOrDog!]]!, $c: Pet) {
            field(a: $a, b: $b, c: $c)
          }
        "# , & [RuleError :: new (& error_message ("a" , "Dog") , & [SourcePosition :: new (25 , 1 , 24)] ,) , RuleError :: new (& error_message ("b" , "[[CatOrDog!]]!") , & [SourcePosition :: new (34 , 1 , 33)] ,) , RuleError :: new (& error_message ("c" , "Pet") , & [SourcePosition :: new (54 , 1 , 53)] ,) ,] ,) ; } }
};
}
