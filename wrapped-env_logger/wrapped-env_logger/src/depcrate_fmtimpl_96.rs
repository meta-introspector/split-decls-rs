// Generated macro for impl_96 (impl)
macro_rules! Depcrate_fmtimpl_96 {
() => {
// Module: crate::fmt
// Provides: {"impl_96"}
// Dependencies: {}
impl Formatter { pub (crate) fn new (writer : & Writer) -> Self { Formatter { buf : Rc :: new (RefCell :: new (writer . buffer ())) , write_style : writer . write_style () , } } pub (crate) fn write_style (& self) -> WriteStyle { self . write_style } pub (crate) fn print (& self , writer : & Writer) -> io :: Result < () > { writer . print (& self . buf . borrow ()) } pub (crate) fn clear (& mut self) { self . buf . borrow_mut () . clear () ; } }
};
}
