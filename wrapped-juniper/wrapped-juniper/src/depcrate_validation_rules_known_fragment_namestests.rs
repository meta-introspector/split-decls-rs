// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_known_fragment_namestests {
() => {
// Module: crate::validation::rules::known_fragment_names
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn known () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            human(id: 4) {
              ...HumanFields1
              ... on Human {
                ...HumanFields2
              }
              ... {
                name
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
        "# ,) ; } # [test] fn unknown () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            human(id: 4) {
              ...UnknownFragment1
              ... on Human {
                ...UnknownFragment2
              }
            }
          }
          fragment HumanFields on Human {
            name
            ...UnknownFragment3
          }
        "# , & [RuleError :: new (& error_message ("UnknownFragment1") , & [SourcePosition :: new (57 , 3 , 17)] ,) , RuleError :: new (& error_message ("UnknownFragment2") , & [SourcePosition :: new (122 , 5 , 19)] ,) , RuleError :: new (& error_message ("UnknownFragment3") , & [SourcePosition :: new (255 , 11 , 15)] ,) ,] ,) ; } }
};
}
