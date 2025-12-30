// Generated macro for parse_into_slice (macro)
macro_rules! Depcrate_decodeparse_into_slice {
() => {
// Module: crate::decode
// Provides: {"parse_into_slice"}
// Dependencies: {}
# [macro_export] macro_rules ! parse_into_slice { ($ data : expr) => { { const HEADER : $ crate :: Header = { let mut data : & [u8] = $ data ; match $ crate :: Header :: parse (& mut data) { Ok (v) => v , Err (_) => panic ! ("Failed to parse items len") , } } ; const ITEMS : [& [u8] ; { HEADER . items_len }] = { match $ crate :: parse_into_array ::< { HEADER . items_len } , { HEADER . dedup_len } > ($ data) { Ok (v) => v , Err (_) => panic ! ("Failed to parse items") , } } ; ITEMS . as_slice () } } ; }
};
}
