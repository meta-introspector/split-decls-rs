// Generated macro for tests (module)
macro_rules! Depcrate_validation_rulestests {
() => {
// Module: crate::validation::rules
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { DefaultScalarValue , parser :: SourcePosition } ; use crate :: validation :: { RuleError , expect_fails_fn } ; # [test] fn handles_recursive_fragments () { expect_fails_fn :: < _ , DefaultScalarValue > (super :: visit_all_rules , "fragment f on QueryRoot { ...f }" , & [RuleError :: new ("Fragment \"f\" is never used" , & [SourcePosition :: new (0 , 0 , 0)] ,) , RuleError :: new ("Cannot spread fragment \"f\"" , & [SourcePosition :: new (26 , 0 , 26)] ,) ,] ,) ; } # [test] fn handles_nested_recursive_fragments () { expect_fails_fn :: < _ , DefaultScalarValue > (super :: visit_all_rules , "fragment f on QueryRoot { a { ...f a { ...f } } }" , & [RuleError :: new ("Fragment \"f\" is never used" , & [SourcePosition :: new (0 , 0 , 0)] ,) , RuleError :: new (r#"Unknown field "a" on type "QueryRoot""# , & [SourcePosition :: new (26 , 0 , 26)] ,) , RuleError :: new ("Cannot spread fragment \"f\"" , & [SourcePosition :: new (30 , 0 , 30)] ,) , RuleError :: new ("Cannot spread fragment \"f\"" , & [SourcePosition :: new (39 , 0 , 39)] ,) ,] ,) ; } }
};
}
