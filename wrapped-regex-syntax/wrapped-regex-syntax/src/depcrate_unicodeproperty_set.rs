// Generated macro for property_set (function)
macro_rules! Depcrate_unicodeproperty_set {
() => {
// Module: crate::unicode
// Provides: {"property_set"}
// Dependencies: {}
# [allow (dead_code)] fn property_set (name_map : & 'static [(& 'static str , Range)] , canonical : & 'static str ,) -> Option < Range > { name_map . binary_search_by_key (& canonical , | x | x . 0) . ok () . map (| i | name_map [i] . 1) }
};
}
