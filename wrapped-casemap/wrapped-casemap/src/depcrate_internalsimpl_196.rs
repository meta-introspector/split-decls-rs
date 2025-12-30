// Generated macro for impl_196 (impl)
macro_rules! Depcrate_internalsimpl_196 {
() => {
// Module: crate::internals
// Provides: {"impl_196"}
// Dependencies: {}
impl < Wr : Writeable > Writeable for StringAndWriteable < '_ , Wr > { fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { sink . write_str (self . string) ? ; self . writeable . write_to (sink) } fn writeable_length_hint (& self) -> writeable :: LengthHint { writeable :: LengthHint :: exact (self . string . len ()) + self . writeable . writeable_length_hint () } }
};
}
