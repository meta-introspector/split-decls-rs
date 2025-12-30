// Generated macro for VariableDefinitionValidator (struct)
macro_rules! Depcrate_transliterate_compile_pass1VariableDefinitionValidator {
() => {
// Module: crate::transliterate::compile::pass1
// Provides: {"VariableDefinitionValidator"}
// Dependencies: {}
# [doc = " Validates variable definitions."] # [doc = ""] # [doc = " This checks that only a limited subset of special constructs appear in a variable's definition."] # [doc = " For example, segments, back references, cursors, anchors, and function calls are not allowed."] # [doc = ""] # [doc = " It also propagates information about whether a variable may appear on the target side of a rule,"] # [doc = " as variables are in general allowed on the target side, but only if they only contain"] # [doc = " special constructs that are allowed to appear on the target side."] struct VariableDefinitionValidator < 'a , 'p , F : Fn (& str) -> bool > { is_variable_defined : F , target_disallowed_variables : & 'a BTreeSet < String > , data : & 'a mut Pass1Data , definition : & 'p [parse :: Element] , used_target_disallowed_construct : bool , }
};
}
