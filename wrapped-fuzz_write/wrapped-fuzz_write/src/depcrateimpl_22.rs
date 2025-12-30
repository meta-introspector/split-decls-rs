// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl Debug for FuzzTestCase < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { if self . initial_junk . is_empty () { writeln ! (f , "let mut writer = ZipWriter::new(Cursor::new(Vec::new()));") ? ; } else { writeln ! (f , "let mut initial_junk = Cursor::new(vec!{:?});\n\
                         initial_junk.seek(SeekFrom::End(0))?;\n\
                         let mut writer = ZipWriter::new(initial_junk);" , & self . initial_junk) ? ; } let _ = self . clone () . execute (f , false) ; Ok (()) } }
};
}
