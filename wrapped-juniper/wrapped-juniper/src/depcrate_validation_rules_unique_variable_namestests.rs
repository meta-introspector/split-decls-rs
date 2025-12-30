// Generated macro for tests (module)
macro_rules! Depcrate_validation_rules_unique_variable_namestests {
() => {
// Module: crate::validation::rules::unique_variable_names
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { error_message , factory } ; use crate :: { parser :: SourcePosition , validation :: { RuleError , expect_fails_rule , expect_passes_rule } , value :: DefaultScalarValue , } ; # [test] fn unique_variable_names () { expect_passes_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query A($x: Int, $y: String) { __typename }
          query B($x: String, $y: Int) { __typename }
        "# ,) ; } # [test] fn duplicate_variable_names () { expect_fails_rule :: < _ , _ , DefaultScalarValue > (factory , r#"
          query A($x: Int, $x: Int, $x: String) { __typename }
          query B($x: String, $x: Int) { __typename }
          query C($x: Int, $x: Int) { __typename }
        "# , & [RuleError :: new (& error_message ("x") , & [SourcePosition :: new (19 , 1 , 18) , SourcePosition :: new (28 , 1 , 27) ,] ,) , RuleError :: new (& error_message ("x") , & [SourcePosition :: new (19 , 1 , 18) , SourcePosition :: new (37 , 1 , 36) ,] ,) , RuleError :: new (& error_message ("x") , & [SourcePosition :: new (82 , 2 , 18) , SourcePosition :: new (94 , 2 , 30) ,] ,) , RuleError :: new (& error_message ("x") , & [SourcePosition :: new (136 , 3 , 18) , SourcePosition :: new (145 , 3 , 27) ,] ,) ,] ,) ; } }
};
}
