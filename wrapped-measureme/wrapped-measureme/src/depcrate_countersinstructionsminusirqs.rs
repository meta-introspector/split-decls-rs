// Generated macro for InstructionsMinusIrqs (struct)
macro_rules! Depcrate_countersInstructionsMinusIrqs {
() => {
// Module: crate::counters
// Provides: {"InstructionsMinusIrqs"}
// Dependencies: {}
# [doc = " More accurate [`Instructions`] (subtracting hardware interrupt counts)."] # [doc = ""] # [doc = " Can be obtained with `Counter::by_name(\"instructions-minus-irqs:u\")`."] pub struct InstructionsMinusIrqs { instructions : hw :: Counter , irqs : hw :: Counter , start : u64 , }
};
}
