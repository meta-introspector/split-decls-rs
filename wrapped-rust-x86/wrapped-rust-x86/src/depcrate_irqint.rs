// Generated macro for int (macro)
macro_rules! Depcrate_irqint {
() => {
// Module: crate::irq
// Provides: {"int"}
// Dependencies: {}
# [doc = " Generate a software interrupt."] # [doc = " This is a macro argument needs to be an immediate."] # [macro_export] macro_rules ! int { ($ x : expr) => { { core :: arch :: asm ! ("int ${vec}" , vec = const ($ x)) ; } } ; }
};
}
