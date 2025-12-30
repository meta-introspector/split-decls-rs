// Generated macro for LineProgram (trait)
macro_rules! Depcrate_read_lineLineProgram {
() => {
// Module: crate::read::line
// Provides: {"LineProgram"}
// Dependencies: {}
# [doc = " A `LineProgram` provides access to a `LineProgramHeader` and"] # [doc = " a way to add files to the files table if necessary. Gimli consumers should"] # [doc = " never need to use or see this trait."] pub trait LineProgram < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { # [doc = " Get a reference to the held `LineProgramHeader`."] fn header (& self) -> & LineProgramHeader < R , Offset > ; # [doc = " Add a file to the file table if necessary."] fn add_file (& mut self , file : FileEntry < R , Offset >) ; }
};
}
