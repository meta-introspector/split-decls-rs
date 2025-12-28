macro_rules! get_loose_u16 {
    () => {
        # [doc = " Avoid monomorphizing multiple copies of this function"] fn get_loose_u16 (payload : & PropertyValueNameToEnumMap < '_ > , name : & str) -> Option < u16 > { fn recurse (mut cursor : ZeroTrieSimpleAsciiCursor , mut rest : & [u8]) -> Option < usize > { if cursor . is_empty () { return None ; } for skip in [b'\t' , b'\n' , b'\x0C' , b'\r' , b' ' , 0x0B , b'_' , b'-'] { let mut skip_cursor = cursor . clone () ; skip_cursor . step (skip) ; if let Some (r) = recurse (skip_cursor , rest) { return Some (r) ; } } let ascii = loop { let Some ((& a , r)) = rest . split_first () else { return cursor . take_value () ; } ; rest = r ; if ! matches ! (a , b'\t' | b'\n' | b'\x0C' | b'\r' | b' ' | 0x0B | b'_' | b'-') { break a ; } } ; let mut other_case_cursor = cursor . clone () ; cursor . step (ascii) ; other_case_cursor . step (if ascii . is_ascii_lowercase () { ascii . to_ascii_uppercase () } else { ascii . to_ascii_lowercase () }) ; recurse (cursor , rest) . or_else (| | recurse (other_case_cursor , rest)) } recurse (payload . map . cursor () , name . as_bytes ()) . and_then (| i | i . try_into () . ok ()) }
    };
}

get_loose_u16!();