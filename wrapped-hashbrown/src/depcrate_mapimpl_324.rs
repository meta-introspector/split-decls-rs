// Generated macro for impl_324 (impl)
macro_rules! Depcrate_mapimpl_324 {
() => {
// Module: crate::map
// Provides: {"impl_324"}
// Dependencies: {}
impl < K , V , A : Allocator > Iterator for IntoKeys < K , V , A > { type Item = K ; # [inline] fn next (& mut self) -> Option < K > { self . inner . next () . map (| (k , _) | k) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [inline] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , (k , _) | f (acc , k)) } }
};
}
