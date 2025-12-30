// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_no_unused_fragmentstests {
() => {
// Module: crate::validation::rules::no_unused_fragments
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn all_fragment_names_are_used () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            human(id: 4) {
              ...HumanFields1
              ... on Human {
                ...HumanFields2
              }
            }
          }
          fragment HumanFields1 on Human {
            name
            ...HumanFields3
          }
          fragment HumanFields2 on Human {
            name
          }
          fragment HumanFields3 on Human {
            name
          }
        "# ,) ; } # [test] fn all_fragment_names_are_used_by_multiple_operations () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            human(id: 4) {
              ...HumanFields1
            }
          }
          query Bar {
            human(id: 4) {
              ...HumanFields2
            }
          }
          fragment HumanFields1 on Human {
            name
            ...HumanFields3
          }
          fragment HumanFields2 on Human {
            name
          }
          fragment HumanFields3 on Human {
            name
          }
        "# ,) ; } # [test] fn contains_unknown_fragments () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            human(id: 4) {
              ...HumanFields1
            }
          }
          query Bar {
            human(id: 4) {
              ...HumanFields2
            }
          }
          fragment HumanFields1 on Human {
            name
            ...HumanFields3
          }
          fragment HumanFields2 on Human {
            name
          }
          fragment HumanFields3 on Human {
            name
          }
          fragment Unused1 on Human {
            name
          }
          fragment Unused2 on Human {
            name
          }
        "# , & [RuleError :: new (& error_message ("Unused1") , & [SourcePosition :: new (465 , 21 , 10)] ,) , RuleError :: new (& error_message ("Unused2") , & [SourcePosition :: new (532 , 24 , 10)] ,) ,] ,) ; } # [test] fn contains_unknown_fragments_with_ref_cycle () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            human(id: 4) {
              ...HumanFields1
            }
          }
          query Bar {
            human(id: 4) {
              ...HumanFields2
            }
          }
          fragment HumanFields1 on Human {
            name
            ...HumanFields3
          }
          fragment HumanFields2 on Human {
            name
          }
          fragment HumanFields3 on Human {
            name
          }
          fragment Unused1 on Human {
            name
            ...Unused2
          }
          fragment Unused2 on Human {
            name
            ...Unused1
          }
        "# , & [RuleError :: new (& error_message ("Unused1") , & [SourcePosition :: new (465 , 21 , 10)] ,) , RuleError :: new (& error_message ("Unused2") , & [SourcePosition :: new (555 , 25 , 10)] ,) ,] ,) ; } # [test] fn contains_unknown_and_undef_fragments () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            human(id: 4) {
              ...bar
            }
          }
          fragment foo on Human {
            name
          }
        "# , & [RuleError :: new (& error_message ("foo") , & [SourcePosition :: new (107 , 6 , 10)] ,)] ,) ; } }
};
}
