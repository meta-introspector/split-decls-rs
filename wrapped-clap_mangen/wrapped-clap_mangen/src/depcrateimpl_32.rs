// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
# [doc = " Handle [`Man`] in relation to files"] impl Man { # [doc = " Generate the filename of the manual page"] # [must_use] pub fn get_filename (& self) -> String { format ! ("{}.{}" , self . cmd . get_display_name () . unwrap_or_else (|| self . cmd . get_name ()) , self . section) } # [doc = " [Renders](Man::render) the manual page and writes it to a file"] pub fn generate_to (& self , out_dir : impl AsRef < std :: path :: Path > ,) -> Result < std :: path :: PathBuf , std :: io :: Error > { let filepath = out_dir . as_ref () . join (self . get_filename ()) ; let mut file = std :: fs :: File :: create (& filepath) ? ; self . render (& mut file) ? ; file . flush () ? ; Ok (filepath) } }
};
}
