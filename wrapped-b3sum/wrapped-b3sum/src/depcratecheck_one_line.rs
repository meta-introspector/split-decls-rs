// Generated macro for check_one_line (function)
macro_rules! Depcratecheck_one_line {
() => {
// Module: crate
// Provides: {"check_one_line"}
// Dependencies: {}
fn check_one_line (line : & str , args : & Args) -> bool { let parse_result = parse_check_line (& line) ; let ParsedCheckLine { file_string , is_escaped , file_path , expected_hash , } = match parse_result { Ok (parsed) => parsed , Err (e) => { eprintln ! ("{}: {}" , NAME , e) ; return false ; } } ; let file_string = if is_escaped { "\\" . to_string () + & file_string } else { file_string } ; let found_hash : blake3 :: Hash ; match hash_path (args , & file_path) { Ok (mut output) => { let mut found_hash_bytes = [0 ; blake3 :: OUT_LEN] ; output . fill (& mut found_hash_bytes) ; found_hash = found_hash_bytes . into () ; } Err (e) => { println ! ("{}: FAILED ({})" , file_string , e) ; return false ; } } ; if expected_hash == found_hash { if ! args . quiet () { println ! ("{}: OK" , file_string) ; } true } else { println ! ("{}: FAILED" , file_string) ; false } }
};
}
