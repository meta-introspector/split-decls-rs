// Generated macro for get_chunks_of_tabs (function)
macro_rules! Depcrate_tabs_in_doc_commentsget_chunks_of_tabs {
() => {
// Module: crate::tabs_in_doc_comments
// Provides: {"get_chunks_of_tabs"}
// Dependencies: {}
# [doc = ""] # [doc = " scans the string for groups of tabs and returns the start(inclusive) and end positions"] # [doc = " (exclusive) of all groups"] # [doc = " e.g. \"sd\\tasd\\t\\taa\" will be converted to [(2, 3), (6, 8)] as"] # [doc = "       012 3456 7 89"] # [doc = "         ^-^  ^---^"] fn get_chunks_of_tabs (the_str : & str) -> Vec < (u32 , u32) > { let line_length_way_to_long = "doc comment longer than 2^32 chars" ; let mut spans : Vec < (u32 , u32) > = vec ! [] ; let mut current_start : u32 = 0 ; let mut is_active = false ; let char_indices : Vec < _ > = the_str . char_indices () . collect () ; if let [(_ , '\t')] = char_indices . as_slice () { return vec ! [(0 , 1)] ; } for entry in char_indices . windows (2) { match entry { [(_ , '\t') , (_ , '\t')] => { is_active = true ; } , [(_ , _) , (index_b , '\t')] => { is_active = true ; current_start = u32 :: try_from (* index_b) . unwrap () ; } , [(_ , '\t') , (index_b , _)] => { is_active = false ; spans . push ((current_start , u32 :: try_from (* index_b) . unwrap ())) ; } , _ => { } , } } if is_active { spans . push ((current_start , u32 :: try_from (char_indices . last () . unwrap () . 0 + 1) . expect (line_length_way_to_long) ,)) ; } spans }
};
}
