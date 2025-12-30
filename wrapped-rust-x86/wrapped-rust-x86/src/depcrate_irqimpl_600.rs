// Generated macro for impl_600 (impl)
macro_rules! Depcrate_irqimpl_600 {
() => {
// Module: crate::irq
// Provides: {"impl_600"}
// Dependencies: {}
impl fmt :: Display for PageFaultError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let p = match self . contains (PageFaultError :: P) { false => "The fault was caused by a non-present page." , true => "The fault was caused by a page-level protection violation." , } ; let wr = match self . contains (PageFaultError :: WR) { false => "The access causing the fault was a read." , true => "The access causing the fault was a write." , } ; let us = match self . contains (PageFaultError :: US) { false => { "The access causing the fault originated when the processor was executing in \
                 supervisor mode." } true => { "The access causing the fault originated when the processor was executing in user \
                 mode." } } ; let rsvd = match self . contains (PageFaultError :: RSVD) { false => "The fault was not caused by reserved bit violation." , true => "The fault was caused by reserved bits set to 1 in a page directory." , } ; let id = match self . contains (PageFaultError :: ID) { false => "The fault was not caused by an instruction fetch." , true => "The fault was caused by an instruction fetch." , } ; write ! (f , "{}\n{}\n{}\n{}\n{}" , p , wr , us , rsvd , id) } }
};
}
