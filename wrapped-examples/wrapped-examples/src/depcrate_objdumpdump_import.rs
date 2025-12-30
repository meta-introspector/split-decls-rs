// Generated macro for dump_import (function)
macro_rules! Depcrate_objdumpdump_import {
() => {
// Module: crate::objdump
// Provides: {"dump_import"}
// Dependencies: {}
fn dump_import < W : Write , E : Write > (w : & mut W , e : & mut E , data : & [u8]) -> Result < () > { let file = match coff :: ImportFile :: parse (data) { Ok (import) => import , Err (err) => { writeln ! (e , "Failed to parse short import: {}" , err) ? ; return Ok (()) ; } } ; writeln ! (w , "Format: Short Import File") ? ; writeln ! (w , "Architecture: {:?}" , file . architecture ()) ? ; if let Some (sub_architecture) = file . sub_architecture () { writeln ! (w , "Sub-Architecture: {:?}" , sub_architecture) ? ; } writeln ! (w , "DLL: {:?}" , String :: from_utf8_lossy (file . dll ())) ? ; writeln ! (w , "Symbol: {:?}" , String :: from_utf8_lossy (file . symbol ())) ? ; write ! (w , "Import: ") ? ; match file . import () { coff :: ImportName :: Ordinal (n) => writeln ! (w , "Ordinal({})" , n) ? , coff :: ImportName :: Name (name) => writeln ! (w , "Name({:?})" , String :: from_utf8_lossy (name)) ? , } writeln ! (w , "Type: {:?}" , file . import_type ()) ? ; Ok (()) }
};
}
