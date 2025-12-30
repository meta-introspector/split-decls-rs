// Generated macro for sorted_iter (function)
macro_rules! Depcratesorted_iter {
() => {
// Module: crate
// Provides: {"sorted_iter"}
// Dependencies: {}
# [doc = " Returns a sorted iterator over a hash map, sorted based on key."] # [doc = ""] # [doc = " The intention of this API is to be used whenever the iteration order of a"] # [doc = " `HashMap` might affect the generated JS bindings. We want to ensure that the"] # [doc = " generated output is deterministic and we do so by ensuring that iteration of"] # [doc = " hash maps is consistently sorted."] fn sorted_iter < K , V > (map : & HashMap < K , V >) -> impl Iterator < Item = (& K , & V) > where K : Ord , { let mut pairs = map . iter () . collect :: < Vec < _ > > () ; pairs . sort_by_key (| (k , _) | * k) ; pairs . into_iter () }
};
}
