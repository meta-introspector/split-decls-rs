// Generated macro for impl_334 (impl)
macro_rules! Depcrate_re_pluginimpl_334 {
() => {
// Module: crate::re_plugin
// Provides: {"impl_334"}
// Dependencies: {}
impl RegularExpression for Plugin { type Text = str ; fn slots_len (& self) -> usize { self . names . len () * 2 } fn next_after_empty (& self , text : & str , i : usize) -> usize { let b = match text . as_bytes () . get (i) { None => return text . len () + 1 , Some (& b) => b , } ; let inc = if b <= 0x7F { 1 } else if b <= 0b110_11111 { 2 } else if b <= 0b1110_1111 { 3 } else { 4 } ; i + inc } fn shortest_match_at (& self , text : & str , start : usize) -> Option < usize > { self . find_at (text , start) . map (| (_ , e) | e) } fn is_match_at (& self , text : & str , start : usize) -> bool { (self . prog) (& mut [] , text , start) } fn find_at (& self , text : & str , start : usize) -> Option < (usize , usize) > { let mut slots = [None , None] ; self . read_captures_at (& mut slots , text , start) } fn read_captures_at < 't > (& self , slots : & mut [Slot] , text : & 't str , start : usize ,) -> Option < (usize , usize) > { for slot in slots . iter_mut () { * slot = None ; } (self . prog) (slots , text , start) ; match (slots [0] , slots [1]) { (Some (s) , Some (e)) => Some ((s , e)) , _ => None , } } }
};
}
