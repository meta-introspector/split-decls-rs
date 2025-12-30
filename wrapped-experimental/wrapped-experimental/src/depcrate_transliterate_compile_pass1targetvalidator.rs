// Generated macro for TargetValidator (struct)
macro_rules! Depcrate_transliterate_compile_pass1TargetValidator {
() => {
// Module: crate::transliterate::compile::pass1
// Provides: {"TargetValidator"}
// Dependencies: {}
# [doc = " Validates the target side of a rule."] # [doc = ""] # [doc = " Ensures that only special constructs (including variables) that may appear on the target side"] # [doc = " of a rule are used. Also validates other target-side-only constraints, such as"] # [doc = " back references not being allowed to overflow and only one cursor being allowed."] struct TargetValidator < 'a , 'p , F : Fn (& str) -> bool > { is_variable_defined : F , target_disallowed_variables : & 'a mut BTreeSet < String > , data : & 'a mut Pass1Data , replacer : & 'p [parse :: Element] , num_segments : u32 , encountered_cursor : bool , }
};
}
