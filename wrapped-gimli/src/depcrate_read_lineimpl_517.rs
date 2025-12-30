// Generated macro for impl_517 (impl)
macro_rules! Depcrate_read_lineimpl_517 {
() => {
// Module: crate::read::line
// Provides: {"impl_517"}
// Dependencies: {}
impl < R , Offset > LineProgram < R , Offset > for IncompleteLineProgram < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { fn header (& self) -> & LineProgramHeader < R , Offset > { & self . header } fn add_file (& mut self , file : FileEntry < R , Offset >) { self . header . file_names . push (file) ; } }
};
}
