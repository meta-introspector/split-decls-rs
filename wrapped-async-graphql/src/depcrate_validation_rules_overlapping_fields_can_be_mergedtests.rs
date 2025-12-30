// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_overlapping_fields_can_be_mergedtests {
() => {
// Module: crate::validation::rules::overlapping_fields_can_be_merged
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; pub fn factory () -> OverlappingFieldsCanBeMerged { OverlappingFieldsCanBeMerged } # [test] fn same_field_on_different_type () { expect_passes_rule ! (factory , r#"
          {
           pet {
            ... on Dog {
                doesKnowCommand(dogCommand: SIT)
            }
            ... on Cat {
                doesKnowCommand(catCommand: JUMP)
            }
           }
          }
        "# ,) ; } # [test] fn same_field_on_same_type () { expect_fails_rule ! (factory , r#"
          {
           pet {
            ... on Dog {
                doesKnowCommand(dogCommand: SIT)
            }
            ... on Dog {
                doesKnowCommand(dogCommand: Heel)
            }
           }
          }
        "# ,) ; } # [test] fn same_alias_on_different_type () { expect_passes_rule ! (factory , r#"
          {
           pet {
            ... on Dog {
                volume: barkVolume
            }
            ... on Cat {
                volume: meowVolume
            }
           }
          }
        "# ,) ; } }
};
}
