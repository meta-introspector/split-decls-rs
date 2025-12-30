// Generated macro for impl_648 (impl)
macro_rules! Depcrate_output_linesimpl_648 {
() => {
// Module: crate::output::lines
// Provides: {"impl_648"}
// Dependencies: {}
impl < 'a > Render < 'a > { pub fn render < W : Write > (mut self , w : & mut W) -> io :: Result < () > { self . filter . sort_files (& mut self . files) ; for file in & self . files { let name_cell = self . render_file (file) ; writeln ! (w , "{}" , ANSIStrings (& name_cell)) ? ; } Ok (()) } fn render_file < 'f > (& self , file : & 'f File < 'a >) -> TextCellContents { self . file_style . for_file (file , self . theme) . with_link_paths () . with_mount_details (false) . paint () } }
};
}
