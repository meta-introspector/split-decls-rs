// Generated macro for unique (function)
macro_rules! Depcrate_unique_implunique {
() => {
// Module: crate::unique_impl
// Provides: {"unique"}
// Dependencies: {}
pub fn unique < I > (iter : I) -> Unique < I > where I : Iterator , I :: Item : Eq + Hash + Clone , { Unique { iter : UniqueBy { iter , used : HashMap :: new () , f : () , } , } }
};
}
