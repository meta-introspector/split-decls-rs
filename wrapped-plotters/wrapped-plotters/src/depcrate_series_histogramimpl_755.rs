// Generated macro for impl_755 (impl)
macro_rules! Depcrate_series_histogramimpl_755 {
() => {
// Module: crate::series::histogram
// Provides: {"impl_755"}
// Dependencies: {}
impl < 'a , BR , A > Iterator for Histogram < 'a , BR , A , Horizontal > where BR : DiscreteRanged , A : AddAssign < A > + Default , { type Item = Rectangle < (A , BR :: ValueType) > ; fn next (& mut self) -> Option < Self :: Item > { while let Some ((y , x)) = self . iter . next () { if let Some ((y , Some (ny))) = self . br . from_index (y) . map (| v | (v , self . br . from_index (y + 1))) { let base = (self . baseline) (& y) ; let style = (self . style) (& y , & x) ; let mut rect = Rectangle :: new ([(x , y) , (base , ny)] , style) ; rect . set_margin (self . margin , self . margin , 0 , 0) ; return Some (rect) ; } } None } }
};
}
