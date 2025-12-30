// Generated macro for InterruptDescription (struct)
macro_rules! Depcrate_irqInterruptDescription {
() => {
// Module: crate::irq
// Provides: {"InterruptDescription"}
// Dependencies: {}
# [doc = " x86 Exception description (see also Intel Vol. 3a Chapter 6)."] # [derive (Debug)] pub struct InterruptDescription { pub vector : u8 , pub mnemonic : & 'static str , pub description : & 'static str , pub irqtype : & 'static str , pub source : & 'static str , }
};
}
