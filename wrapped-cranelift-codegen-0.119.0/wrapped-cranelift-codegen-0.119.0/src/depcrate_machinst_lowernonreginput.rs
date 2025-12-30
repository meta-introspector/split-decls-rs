// Generated macro for NonRegInput (struct)
macro_rules! Depcrate_machinst_lowerNonRegInput {
() => {
// Module: crate::machinst::lower
// Provides: {"NonRegInput"}
// Dependencies: {}
# [doc = " A representation of all of the ways in which a value is available, aside"] # [doc = " from as a direct register."] # [doc = ""] # [doc = " - An instruction, if it would be allowed to occur at the current location"] # [doc = "   instead (see [Lower::get_input_as_source_or_const()] for more details)."] # [doc = ""] # [doc = " - A constant, if the value is known to be a constant."] # [derive (Clone , Copy , Debug)] pub struct NonRegInput { # [doc = " An instruction produces this value (as the given output), and its"] # [doc = " computation (and side-effect if applicable) could occur at the"] # [doc = " current instruction's location instead."] # [doc = ""] # [doc = " If this instruction's operation is merged into the current instruction,"] # [doc = " the backend must call [Lower::sink_inst()]."] # [doc = ""] # [doc = " This enum indicates whether this use of the source instruction"] # [doc = " is unique or not."] pub inst : InputSourceInst , # [doc = " The value is a known constant."] pub constant : Option < u64 > , }
};
}
