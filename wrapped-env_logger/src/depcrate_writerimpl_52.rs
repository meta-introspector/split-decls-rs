// Generated macro for impl_52 (impl)
macro_rules! Depcrate_writerimpl_52 {
() => {
// Module: crate::writer
// Provides: {"impl_52"}
// Dependencies: {}
impl Writer { pub (crate) fn write_style (& self) -> WriteStyle { self . inner . write_style () } pub (crate) fn buffer (& self) -> Buffer { self . inner . buffer () } pub (crate) fn print (& self , buf : & Buffer) -> io :: Result < () > { self . inner . print (buf) } }
};
}
