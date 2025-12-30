// Generated macro for impl_754 (impl)
macro_rules! Depcrate_series_histogramimpl_754 {
() => {
// Module: crate::series::histogram
// Provides: {"impl_754"}
// Dependencies: {}
impl < 'a , BR , A > Iterator for Histogram < 'a , BR , A , Vertical > where BR : DiscreteRanged , A : AddAssign < A > + Default , { type Item = Rectangle < (BR :: ValueType , A) > ; fn next (& mut self) -> Option < Self :: Item > { while let Some ((x , y)) = self . iter . next () { if let Some ((x , Some (nx))) = self . br . from_index (x) . map (| v | (v , self . br . from_index (x + 1))) { let base = (self . baseline) (& x) ; let style = (self . style) (& x , & y) ; let mut rect = Rectangle :: new ([(x , y) , (nx , base)] , style) ; rect . set_margin (0 , 0 , self . margin , self . margin) ; return Some (rect) ; } } None } }
};
}
