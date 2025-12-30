// Generated macro for CaseTransform (enum)
macro_rules! DepcrateCaseTransform {
() => {
// Module: crate
// Provides: {"CaseTransform"}
// Dependencies: {}
# [doc = " Type that knows how to transform the case of individual option flag names."] # [derive (Debug , Default , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum CaseTransform { # [doc = " Do not transform the name."] # [default] Identity , # [doc = " `lower_snake_case`"] LowerSnake , # [doc = " `UPPER_SNAKE_CASE`"] UpperSnake , # [doc = " `lowerCamelCase`"] LowerCamel , # [doc = " `UpperCamelCase`"] UpperCamel , # [doc = " `kebab-case`"] Kebab , # [doc = " `Title Case`"] Title , }
};
}
