// Generated macro for ExceptionSpec (enum)
macro_rules! Depcrate_astExceptionSpec {
() => {
// Module: crate::ast
// Provides: {"ExceptionSpec"}
// Dependencies: {}
# [doc = " The `<exception-spec>` production."] # [doc = ""] # [doc = " <exception-spec> ::= Do                # non-throwing exception-specification (e.g., noexcept, throw())"] # [doc = "                  ::= DO <expression> E # computed (instantiation-dependent) noexcept"] # [doc = "                  ::= Dw <type>+ E      # dynamic exception specification with instantiation-dependent types"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum ExceptionSpec { # [doc = " noexcept"] NoExcept , # [doc = " noexcept(expression)"] Computed (Expression) , }
};
}
