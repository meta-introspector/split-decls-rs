// Generated macro for impl_208 (impl)
macro_rules! Depcrate_info_sourcesimpl_208 {
() => {
// Module: crate::info::sources
// Provides: {"impl_208"}
// Dependencies: {}
impl < 'a > File < 'a > { # [doc = " For this file, return a vector of alternate file paths that, if any of"] # [doc = " them exist, mean that *this* file should be coloured as “compiled”."] # [doc = ""] # [doc = " The point of this is to highlight compiled files such as `foo.js` when"] # [doc = " their source file `foo.coffee` exists in the same directory."] # [doc = " For example, `foo.js` is perfectly valid without `foo.coffee`, so we"] # [doc = " don’t want to always blindly highlight `*.js` as compiled."] # [doc = " (See also `FileType`)"] pub fn get_source_files (& self) -> Vec < PathBuf > { if let Some (ext) = & self . ext { match & ext [..] { "css" => vec ! [self . path . with_extension ("sass") , self . path . with_extension ("scss") , self . path . with_extension ("styl") , self . path . with_extension ("less")] , "mjs" => vec ! [self . path . with_extension ("mts")] , "cjs" => vec ! [self . path . with_extension ("cts")] , "js" => vec ! [self . path . with_extension ("coffee") , self . path . with_extension ("ts")] , "aux" | "bbl" | "bcf" | "blg" | "fdb_latexmk" | "fls" | "headfootlength" | "lof" | "log" | "lot" | "out" | "toc" | "xdv" => vec ! [self . path . with_extension ("tex")] , _ => vec ! [] , } } else { vec ! [] } } }
};
}
