// Generated macro for two_elements_of (function)
macro_rules! Depcrate_builder_commandtwo_elements_of {
() => {
// Module: crate::builder::command
// Provides: {"two_elements_of"}
// Dependencies: {}
# [doc = " Returns the first two elements of an iterator as an `Option<(T, T)>`."] # [doc = ""] # [doc = " If the iterator has fewer than two elements, it returns `None`."] fn two_elements_of < I , T > (mut iter : I) -> Option < (T , T) > where I : Iterator < Item = T > , { let first = iter . next () ; let second = iter . next () ; match (first , second) { (Some (first) , Some (second)) => Some ((first , second)) , _ => None , } }
};
}
