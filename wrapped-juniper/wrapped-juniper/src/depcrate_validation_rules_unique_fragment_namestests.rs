// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_unique_fragment_namestests {
() => {
// Module: crate::validation::rules::unique_fragment_names
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { duplicate_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn no_fragments () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog {
              name
            }
          }
        "# ,) ; } # [test] fn one_fragment () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog {
              ...fragA
            }
          }

          fragment fragA on Dog {
            name
          }
        "# ,) ; } # [test] fn many_fragments () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog {
              ...fragA
              ...fragB
              ...fragC
            }
          }
          fragment fragA on Dog {
            name
          }
          fragment fragB on Dog {
            nickname
          }
          fragment fragC on Dog {
            barkVolume
          }
        "# ,) ; } # [test] fn inline_fragments_always_unique () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dorOrHuman {
              ...on Dog {
                name
              }
              ...on Dog {
                barkVolume
              }
            }
          }
        "# ,) ; } # [test] fn fragment_and_operation_named_the_same () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query Foo {
            dog {
              ...Foo
            }
          }
          fragment Foo on Dog {
            name
          }
        "# ,) ; } # [test] fn fragments_named_the_same () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            dog {
              ...fragA
            }
          }
          fragment fragA on Dog {
            name
          }
          fragment fragA on Dog {
            barkVolume
          }
        "# , & [RuleError :: new (& duplicate_message ("fragA") , & [SourcePosition :: new (99 , 6 , 19) , SourcePosition :: new (162 , 9 , 19) ,] ,)] ,) ; } # [test] fn fragments_named_the_same_no_reference () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          fragment fragA on Dog {
            name
          }
          fragment fragA on Dog {
            barkVolume
          }
        "# , & [RuleError :: new (& duplicate_message ("fragA") , & [SourcePosition :: new (20 , 1 , 19) , SourcePosition :: new (83 , 4 , 19) ,] ,)] ,) ; } }
};
}
