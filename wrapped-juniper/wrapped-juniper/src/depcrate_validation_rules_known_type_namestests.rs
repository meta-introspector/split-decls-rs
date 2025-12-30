// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_known_type_namestests {
() => {
// Module: crate::validation::rules::known_type_names
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn known_type_names_are_valid () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($var: String, $required: [String!]!) {
            user(id: 4) {
              pets { ... on Pet { name }, ...PetFields, ... { name } }
            }
          }
          fragment PetFields on Pet {
            name
          }
        "# ,) ; } # [test] fn unknown_type_names_are_invalid () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo($var: JumbledUpLetters) {
            user(id: 4) {
              name
              pets { ... on Badger { name }, ...PetFields }
            }
          }
          fragment PetFields on Peettt {
            name
          }
        "# , & [RuleError :: new (& error_message ("JumbledUpLetters") , & [SourcePosition :: new (27 , 1 , 26)] ,) , RuleError :: new (& error_message ("Badger") , & [SourcePosition :: new (120 , 4 , 28)]) , RuleError :: new (& error_message ("Peettt") , & [SourcePosition :: new (210 , 7 , 32)]) ,] ,) ; } }
};
}
