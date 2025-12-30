// Generated macro for ModularRefOps (trait)
macro_rules! DepcrateModularRefOps {
() => {
// Module: crate
// Provides: {"ModularRefOps"}
// Dependencies: {}
# [doc = " Collection of operations similar to [ModularOps], but takes operands with references"] pub trait ModularRefOps : for < 'r > ModularOps < & 'r Self , & 'r Self > + Sized { }
};
}
