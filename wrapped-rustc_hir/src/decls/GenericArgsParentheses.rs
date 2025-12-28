macro_rules! GenericArgsParentheses {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug , HashStable_Generic)] pub enum GenericArgsParentheses { No , # [doc = " Bounds for `feature(return_type_notation)`, like `T: Trait<method(..): Send>`,"] # [doc = " where the args are explicitly elided with `..`"] ReturnTypeNotation , # [doc = " parenthesized function-family traits, like `T: Fn(u32) -> i32`"] ParenSugar , }
    };
}

GenericArgsParentheses!();