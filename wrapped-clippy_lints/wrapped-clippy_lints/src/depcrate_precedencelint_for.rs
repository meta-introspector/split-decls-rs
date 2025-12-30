// Generated macro for lint_for (function)
macro_rules! Depcrate_precedencelint_for {
() => {
// Module: crate::precedence
// Provides: {"lint_for"}
// Dependencies: {}
fn lint_for (ops : & [BinOpKind]) -> & 'static Lint { if ops . iter () . all (| op | is_bit_op (* op)) { PRECEDENCE_BITS } else { PRECEDENCE } }
};
}
