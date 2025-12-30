// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Box < dyn Error + Send + Sync > > { let opt = Opt :: parse () ; match opt { Opt :: Truncate (opt) => { let file_contents = std :: fs :: read (& opt . file) ? ; let truncated = truncate (& file_contents) ? ; let output_file_name = opt . file . with_extension ("truncated.mm_profdata") ; std :: fs :: write (output_file_name , truncated) ? ; } } Ok (()) }
};
}
