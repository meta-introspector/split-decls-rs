// Generated macro for impl_330 (impl)
macro_rules! Depcrate_mapimpl_330 {
() => {
// Module: crate::map
// Provides: {"impl_330"}
// Dependencies: {}
impl < K , V , A : Allocator > Iterator for IntoValues < K , V , A > { type Item = V ; # [inline] fn next (& mut self) -> Option < V > { self . inner . next () . map (| (_ , v) | v) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [inline] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , (_ , v) | f (acc , v)) } }
};
}
