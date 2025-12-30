// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_no_fragment_cyclestests {
() => {
// Module: crate::validation::rules::no_fragment_cycles
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn single_reference_is_valid () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Dog { ...fragB }
          fragment fragB on Dog { name }
        "# ,) ; } # [test] fn spreading_twice_is_not_circular () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Dog { ...fragB, ...fragB }
          fragment fragB on Dog { name }
        "# ,) ; } # [test] fn spreading_twice_indirectly_is_not_circular () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Dog { ...fragB, ...fragC }
          fragment fragB on Dog { ...fragC }
          fragment fragC on Dog { name }
        "# ,) ; } # [test] fn double_spread_within_abstract_types () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment nameFragment on Pet {
            ... on Dog { name }
            ... on Cat { name }
          }

          fragment spreadsInAnon on Pet {
            ... on Dog { ...nameFragment }
            ... on Cat { ...nameFragment }
          }
        "# ,) ; } # [test] fn does_not_false_positive_on_unknown_fragment () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment nameFragment on Pet {
            ...UnknownFragment
          }
        "# ,) ; } # [test] fn spreading_recursively_within_field_fails () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Human { relatives { ...fragA } },
        "# , & [RuleError :: new (& error_message ("fragA") , & [SourcePosition :: new (49 , 1 , 48)] ,)] ,) ; } # [test] fn no_spreading_itself_directly () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Dog { ...fragA }
        "# , & [RuleError :: new (& error_message ("fragA") , & [SourcePosition :: new (35 , 1 , 34)] ,)] ,) ; } # [test] fn no_spreading_itself_directly_within_inline_fragment () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Pet {
            ... on Dog {
              ...fragA
            }
          }
        "# , & [RuleError :: new (& error_message ("fragA") , & [SourcePosition :: new (74 , 3 , 14)] ,)] ,) ; } # [test] fn no_spreading_itself_indirectly () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Dog { ...fragB }
          fragment fragB on Dog { ...fragA }
        "# , & [RuleError :: new (& error_message ("fragA") , & [SourcePosition :: new (35 , 1 , 34)] ,)] ,) ; } # [test] fn no_spreading_itself_indirectly_reports_opposite_order () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragB on Dog { ...fragA }
          fragment fragA on Dog { ...fragB }
        "# , & [RuleError :: new (& error_message ("fragB") , & [SourcePosition :: new (35 , 1 , 34)] ,)] ,) ; } # [test] fn no_spreading_itself_indirectly_within_inline_fragment () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Pet {
            ... on Dog {
              ...fragB
            }
          }
          fragment fragB on Pet {
            ... on Dog {
              ...fragA
            }
          }
        "# , & [RuleError :: new (& error_message ("fragA") , & [SourcePosition :: new (74 , 3 , 14)] ,)] ,) ; } # [test] fn no_spreading_itself_deeply () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Dog { ...fragB }
          fragment fragB on Dog { ...fragC }
          fragment fragC on Dog { ...fragO }
          fragment fragX on Dog { ...fragY }
          fragment fragY on Dog { ...fragZ }
          fragment fragZ on Dog { ...fragO }
          fragment fragO on Dog { ...fragP }
          fragment fragP on Dog { ...fragA, ...fragX }
        "# , & [RuleError :: new (& error_message ("fragA") , & [SourcePosition :: new (35 , 1 , 34)]) , RuleError :: new (& error_message ("fragO") , & [SourcePosition :: new (305 , 7 , 34)]) ,] ,) ; } # [test] fn no_spreading_itself_deeply_two_paths () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Dog { ...fragB, ...fragC }
          fragment fragB on Dog { ...fragA }
          fragment fragC on Dog { ...fragA }
        "# , & [RuleError :: new (& error_message ("fragA") , & [SourcePosition :: new (35 , 1 , 34)]) , RuleError :: new (& error_message ("fragA") , & [SourcePosition :: new (45 , 1 , 44)]) ,] ,) ; } # [test] fn no_spreading_itself_deeply_two_paths_alt_traversal_order () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Dog { ...fragC }
          fragment fragB on Dog { ...fragC }
          fragment fragC on Dog { ...fragA, ...fragB }
        "# , & [RuleError :: new (& error_message ("fragA") , & [SourcePosition :: new (35 , 1 , 34)]) , RuleError :: new (& error_message ("fragC") , & [SourcePosition :: new (135 , 3 , 44)]) ,] ,) ; } # [test] fn no_spreading_itself_deeply_and_immediately () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Dog { ...fragB }
          fragment fragB on Dog { ...fragB, ...fragC }
          fragment fragC on Dog { ...fragA, ...fragB }
        "# , & [RuleError :: new (& error_message ("fragA") , & [SourcePosition :: new (35 , 1 , 34)]) , RuleError :: new (& error_message ("fragB") , & [SourcePosition :: new (80 , 2 , 34)]) , RuleError :: new (& error_message ("fragB") , & [SourcePosition :: new (90 , 2 , 44)]) ,] ,) ; } }
};
}
