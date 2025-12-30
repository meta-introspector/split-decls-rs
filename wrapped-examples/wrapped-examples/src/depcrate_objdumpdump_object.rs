// Generated macro for dump_object (function)
macro_rules! Depcrate_objdumpdump_object {
() => {
// Module: crate::objdump
// Provides: {"dump_object"}
// Dependencies: {}
fn dump_object < W : Write , E : Write > (w : & mut W , e : & mut E , data : & [u8]) -> Result < () > { match object :: File :: parse (data) { Ok (file) => { dump_parsed_object (w , e , & file) ? ; } Err (err) => { writeln ! (e , "Failed to parse file: {}" , err) ? ; } } Ok (()) }
};
}
