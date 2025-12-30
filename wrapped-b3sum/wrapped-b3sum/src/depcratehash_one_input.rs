// Generated macro for hash_one_input (function)
macro_rules! Depcratehash_one_input {
() => {
// Module: crate
// Provides: {"hash_one_input"}
// Dependencies: {}
fn hash_one_input (path : & Path , args : & Args) -> anyhow :: Result < () > { let output = hash_path (args , path) ? ; if args . raw () { write_raw_output (output , args) ? ; return Ok (()) ; } if args . no_names () { write_hex_output (output , args) ? ; println ! () ; return Ok (()) ; } let FilepathString { filepath_string , is_escaped , } = filepath_to_string (path) ; if is_escaped { print ! ("\\") ; } if args . tag () { print ! ("BLAKE3 ({}) = " , filepath_string) ; write_hex_output (output , args) ? ; println ! () ; return Ok (()) ; } write_hex_output (output , args) ? ; println ! ("  {}" , filepath_string) ; Ok (()) }
};
}
