// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_variables_are_input_typestests {
() => {
// Module: crate::validation::rules::variables_are_input_types
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; pub fn factory () -> VariablesAreInputTypes { VariablesAreInputTypes } # [test] fn input_types_are_valid () { expect_passes_rule ! (factory , r#"
          query Foo($a: String, $b: [Boolean!]!, $c: ComplexInput) {
            field(a: $a, b: $b, c: $c)
          }
        "# ,) ; } # [test] fn output_types_are_invalid () { expect_fails_rule ! (factory , r#"
          query Foo($a: Dog, $b: [[CatOrDog!]]!, $c: Pet) {
            field(a: $a, b: $b, c: $c)
          }
        "# ,) ; } }
};
}
