// Generated macro for mutation (module)
macro_rules! Depcrate_filemutation {
() => {
// Module: crate::file
// Provides: {"mutation"}
// Dependencies: {}
mod mutation { use std :: path :: PathBuf ; use crate :: File ; # [doc = " Mutating access"] impl File { # [doc = " Set the path at which we think we are located to the given `path`."] # [doc = ""] # [doc = " This is useful to change the location of the index *once* it is written via [`write()`][File::write()]."] pub fn set_path (& mut self , path : impl Into < PathBuf >) { self . path = path . into () ; } } }
};
}
