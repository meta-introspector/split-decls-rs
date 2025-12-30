// Generated macro for impl_577 (impl)
macro_rules! Depcrate_irqimpl_577 {
() => {
// Module: crate::irq
// Provides: {"impl_577"}
// Dependencies: {}
impl fmt :: Display for InterruptDescription { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{} ({}, vec={}) {}" , self . mnemonic , self . irqtype , self . vector , self . description) } }
};
}
