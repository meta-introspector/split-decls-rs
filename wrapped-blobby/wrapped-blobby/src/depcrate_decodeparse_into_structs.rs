// Generated macro for parse_into_structs (macro)
macro_rules! Depcrate_decodeparse_into_structs {
() => {
// Module: crate::decode
// Provides: {"parse_into_structs"}
// Dependencies: {}
# [macro_export] macro_rules ! parse_into_structs { ($ data : expr ; # [define_struct] $ static_vis : vis static $ items_name : ident : & [$ ty_vis : vis $ item : ident { $ ($ field : ident) ,* $ (,) ? }] ;) => { # [derive (Debug , Clone , Copy , Eq , PartialEq)] $ ty_vis struct $ item { pub $ ($ field : &'static [u8]) ,* } $ crate :: parse_into_structs ! ($ data ; $ static_vis static $ items_name : & [$ item { $ ($ field) ,* }] ;) ; } ; ($ data : expr ; $ static_vis : vis static $ items_name : ident : & [$ item : ident { $ ($ field : ident) ,* $ (,) ? }] ;) => { $ static_vis static $ items_name : & [$ item] = { const RAW_ITEMS : & [& [u8]] = $ crate :: parse_into_slice ! ($ data) ; const fn get_struct (items : & mut & [&'static [u8]]) -> $ item { $ item { $ ($ field : { match items . split_first () { Some ((first , rest)) => { * items = rest ; first } None => unreachable ! () , } }) ,* } } const ITEM_FIELDS : usize = 0 $ (+ { let $ field : () ; let _ = $ field ; 1 }) *; const ITEMS_LEN : usize = if RAW_ITEMS . len () % ITEM_FIELDS == 0 { RAW_ITEMS . len () / ITEM_FIELDS } else { panic ! ("Number of raw items is not multiple of number of fields in the struct") ; } ; const ITEMS : [$ item ; ITEMS_LEN] = { let mut res = [$ item { $ ($ field : & []) ,* } ; ITEMS_LEN] ; let mut raw_items = RAW_ITEMS ; let mut i = 0 ; while i < res . len () { res [i] = get_struct (& mut raw_items) ; i += 1 ; } res } ; ITEMS . as_slice () } ; } ; }
};
}
