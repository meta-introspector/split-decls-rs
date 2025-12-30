// Generated macro for list_item_padding_of (function)
macro_rules! Depcrate_text_modificationslist_item_padding_of {
() => {
// Module: crate::text_modifications
// Provides: {"list_item_padding_of"}
// Dependencies: {}
pub (crate) fn list_item_padding_of (l : Option < u64 >) -> Cow < 'static , str > { match l { None => "  " . into () , Some (n) => format ! ("{n}. ") . chars () . map (| _ | ' ') . collect :: < String > () . into () , } }
};
}
