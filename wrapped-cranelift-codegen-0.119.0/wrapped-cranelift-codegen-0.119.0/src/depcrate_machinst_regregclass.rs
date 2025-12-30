// Generated macro for RegClass (type)
macro_rules! Depcrate_machinst_regRegClass {
() => {
// Module: crate::machinst::reg
// Provides: {"RegClass"}
// Dependencies: {}
# [doc = " A register class. Each register in the ISA has one class, and the"] # [doc = " classes are disjoint. Most modern ISAs will have just two classes:"] # [doc = " the integer/general-purpose registers (GPRs), and the float/vector"] # [doc = " registers (typically used for both)."] # [doc = ""] # [doc = " Note that unlike some other compiler backend/register allocator"] # [doc = " designs, we do not allow for overlapping classes, i.e. registers"] # [doc = " that belong to more than one class, because doing so makes the"] # [doc = " allocation problem significantly more complex. Instead, when a"] # [doc = " register can be addressed under different names for different"] # [doc = " sizes (for example), the backend author should pick classes that"] # [doc = " denote some fundamental allocation unit that encompasses the whole"] # [doc = " register. For example, always allocate 128-bit vector registers"] # [doc = " `v0`..`vN`, even though `f32` and `f64` values may use only the"] # [doc = " low 32/64 bits of those registers and name them differently."] pub type RegClass = regalloc2 :: RegClass ;
};
}
