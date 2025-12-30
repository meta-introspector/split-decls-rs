// Generated macro for impl_30 (impl)
macro_rules! Depcrate_utilsimpl_30 {
() => {
// Module: crate::utils
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a , I > Iterator for TextMergeWithOffset < 'a , I > where I : Iterator < Item = (Event < 'a > , Range < usize >) > , { type Item = (Event < 'a > , Range < usize >) ; fn next (& mut self) -> Option < Self :: Item > { match (self . last_event . take () , self . iter . next ()) { (Some ((Event :: Text (last_text) , last_offset)) , Some ((Event :: Text (next_text) , next_offset)) ,) => { let mut string_buf : String = last_text . into_string () ; string_buf . push_str (& next_text) ; let mut offset = last_offset ; offset . end = next_offset . end ; loop { match self . iter . next () { Some ((Event :: Text (next_text) , next_offset)) => { string_buf . push_str (& next_text) ; offset . end = next_offset . end ; } next_event => { self . last_event = next_event ; if string_buf . is_empty () { break self . next () ; } else { break Some ((Event :: Text (CowStr :: Boxed (string_buf . into_boxed_str ())) , offset ,)) ; } } } } } (None , Some (next_event)) => { self . last_event = Some (next_event) ; self . next () } (None , None) => { None } (last_event , next_event) => { self . last_event = next_event ; last_event } } } }
};
}
