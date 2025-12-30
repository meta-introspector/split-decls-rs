// Generated macro for intrinsic_args (macro)
macro_rules! Depcrate_intrinsicsintrinsic_args {
() => {
// Module: crate::intrinsics
// Provides: {"intrinsic_args"}
// Dependencies: {}
macro_rules ! intrinsic_args { ($ fx : expr , $ args : expr => ($ ($ arg : tt) ,*) ; $ intrinsic : expr) => { # [allow (unused_parens)] let ($ ($ arg) ,*) = if let [$ ($ arg) ,*] = $ args { ($ (codegen_operand ($ fx , & ($ arg) . node)) ,*) } else { $ crate :: intrinsics :: bug_on_incorrect_arg_count ($ intrinsic) ; } ; } }
};
}
