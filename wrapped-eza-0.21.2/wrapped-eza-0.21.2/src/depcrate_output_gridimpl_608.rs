// Generated macro for impl_608 (impl)
macro_rules! Depcrate_output_gridimpl_608 {
() => {
// Module: crate::output::grid
// Provides: {"impl_608"}
// Dependencies: {}
impl < 'a > Render < 'a > { pub fn render < W : Write > (mut self , w : & mut W) -> io :: Result < () > { self . filter . sort_files (& mut self . files) ; let cells = self . files . iter () . map (| file | { self . file_style . for_file (file , self . theme) . paint () . strings () . to_string () }) . collect () ; let grid = Grid :: new (cells , GridOptions { filling : Filling :: Spaces (2) , direction : self . opts . direction () , width : self . console_width , } ,) ; write ! (w , "{grid}") } }
};
}
