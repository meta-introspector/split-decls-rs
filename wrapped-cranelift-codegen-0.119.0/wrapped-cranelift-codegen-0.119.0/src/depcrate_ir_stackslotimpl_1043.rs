// Generated macro for impl_1043 (impl)
macro_rules! Depcrate_ir_stackslotimpl_1043 {
() => {
// Module: crate::ir::stackslot
// Provides: {"impl_1043"}
// Dependencies: {}
impl fmt :: Display for StackSlotData { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . align_shift != 0 { write ! (f , "{} {}, align = {}" , self . kind , self . size , 1u32 << self . align_shift) } else { write ! (f , "{} {}" , self . kind , self . size) } } }
};
}
