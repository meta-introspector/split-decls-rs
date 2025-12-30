// Generated macro for build_map (function)
macro_rules! Depcratebuild_map {
() => {
// Module: crate
// Provides: {"build_map"}
// Dependencies: {}
fn build_map (entries : & [Entry] , state : HashState) -> proc_macro2 :: TokenStream { let key = state . key ; let disps = state . disps . iter () . map (| & (d1 , d2) | quote ! ((# d1 , # d2))) ; let entries = state . map . iter () . map (| & idx | { let entry = & entries [idx] ; let key = & entry . key . expr [0] ; let value = & entry . value ; quote ! ((# key , # value)) }) ; quote ! { phf :: Map { key : # key , disps : & [# (# disps) ,*] , entries : & [# (# entries) ,*] , } } }
};
}
