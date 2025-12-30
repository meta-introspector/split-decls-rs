// Generated macro for impl_518 (impl)
macro_rules! Depcrate_read_lineimpl_518 {
() => {
// Module: crate::read::line
// Provides: {"impl_518"}
// Dependencies: {}
impl < 'program , R , Offset > LineProgram < R , Offset > for & 'program CompleteLineProgram < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { fn header (& self) -> & LineProgramHeader < R , Offset > { & self . header } fn add_file (& mut self , _ : FileEntry < R , Offset >) { } }
};
}
