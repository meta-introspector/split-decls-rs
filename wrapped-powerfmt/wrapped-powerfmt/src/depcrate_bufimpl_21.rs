// Generated macro for impl_21 (impl)
macro_rules! Depcrate_bufimpl_21 {
() => {
// Module: crate::buf
// Provides: {"impl_21"}
// Dependencies: {}
impl < const SIZE : usize > SmartDisplay for WriteBuffer < SIZE > { type Metadata = () ; fn metadata (& self , _ : FormatterOptions) -> Metadata < '_ , Self > { Metadata :: new (self . len , self , ()) } fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (self) } }
};
}
