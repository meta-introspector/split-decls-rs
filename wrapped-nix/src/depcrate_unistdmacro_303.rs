// Generated macro for macro_303 (macro)
macro_rules! Depcrate_unistdmacro_303 {
() => {
// Module: crate::unistd
// Provides: {"macro_303"}
// Dependencies: {}
feature ! { #! [all (feature = "fs")] # [doc = " Set the file flags."] # [doc = ""] # [doc = " See also [chflags(2)](https://www.freebsd.org/cgi/man.cgi?query=chflags&sektion=2)"] # [cfg (bsd)] pub fn chflags < P : ? Sized + NixPath > (path : & P , flags : FileFlag) -> Result < () > { let res = path . with_nix_path (| cstr | unsafe { libc :: chflags (cstr . as_ptr () , flags . bits ()) }) ?; Errno :: result (res) . map (drop) } }
};
}
