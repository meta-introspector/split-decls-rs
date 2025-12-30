// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_unique_input_field_namestests {
() => {
// Module: crate::validation::rules::unique_input_field_names
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn input_object_with_fields () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            field(arg: { f: true })
          }
        "# ,) ; } # [test] fn same_input_object_within_two_args () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            field(arg1: { f: true }, arg2: { f: true })
          }
        "# ,) ; } # [test] fn multiple_input_object_fields () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            field(arg: { f1: "value", f2: "value", f3: "value" })
          }
        "# ,) ; } # [test] fn allows_for_nested_input_objects_with_similar_fields () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            field(arg: {
              deep: {
                deep: {
                  id: 1
                }
                id: 1
              }
              id: 1
            })
          }
        "# ,) ; } # [test] fn duplicate_input_object_fields () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            field(arg: { f1: "value", f1: "value" })
          }
        "# , & [RuleError :: new (& error_message ("f1") , & [SourcePosition :: new (38 , 2 , 25) , SourcePosition :: new (51 , 2 , 38) ,] ,)] ,) ; } # [test] fn many_duplicate_input_object_fields () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          {
            field(arg: { f1: "value", f1: "value", f1: "value" })
          }
        "# , & [RuleError :: new (& error_message ("f1") , & [SourcePosition :: new (38 , 2 , 25) , SourcePosition :: new (51 , 2 , 38) ,] ,) , RuleError :: new (& error_message ("f1") , & [SourcePosition :: new (38 , 2 , 25) , SourcePosition :: new (64 , 2 , 51) ,] ,) ,] ,) ; } }
};
}
