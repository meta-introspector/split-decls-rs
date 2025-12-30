// Generated macro for build_ordered_map (function)
macro_rules! Depcratebuild_ordered_map {
() => {
// Module: crate
// Provides: {"build_ordered_map"}
// Dependencies: {}
fn build_ordered_map (entries : & [Entry] , state : HashState) -> proc_macro2 :: TokenStream { let key = state . key ; let disps = state . disps . iter () . map (| & (d1 , d2) | quote ! ((# d1 , # d2))) ; let idxs = state . map . iter () . map (| idx | quote ! (# idx)) ; let entries = entries . iter () . map (| entry | { let key = & entry . key . expr [0] ; let value = & entry . value ; quote ! ((# key , # value)) }) ; quote ! { phf :: OrderedMap { key : # key , disps : & [# (# disps) ,*] , idxs : & [# (# idxs) ,*] , entries : & [# (# entries) ,*] , } } }
};
}
