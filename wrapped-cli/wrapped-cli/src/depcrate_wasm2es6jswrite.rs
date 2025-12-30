// Generated macro for write (function)
macro_rules! Depcrate_wasm2es6jswrite {
() => {
// Module: crate::wasm2es6js
// Provides: {"write"}
// Dependencies: {}
fn write (args : & Args , extension : & str , contents : & [u8] , print_fallback : bool) -> Result < () , Error > { if let Some (p) = & args . output { let dst = p . with_extension (extension) ; fs :: write (& dst , contents) . with_context (| | format ! ("failed to write `{}`" , dst . display ())) ? ; } else if let Some (p) = & args . out_dir { let filename = args . input . file_name () . unwrap () ; let dst = p . join (filename) . with_extension (extension) ; fs :: write (& dst , contents) . with_context (| | format ! ("failed to write `{}`" , dst . display ())) ? ; } else if print_fallback { println ! ("{}" , String :: from_utf8_lossy (contents)) } Ok (()) }
};
}
