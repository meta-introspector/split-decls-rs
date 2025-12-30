// Generated macro for impl_276 (impl)
macro_rules! Depcrate_pathimpl_276 {
() => {
// Module: crate::path
// Provides: {"impl_276"}
// Dependencies: {}
impl CGPathElement { pub fn points (& self) -> & [CGPoint] { unsafe { match self . element_type { CGPathElementType :: CloseSubpath => & [] , CGPathElementType :: MoveToPoint | CGPathElementType :: AddLineToPoint => { slice :: from_raw_parts (self . points , 1) } CGPathElementType :: AddQuadCurveToPoint => slice :: from_raw_parts (self . points , 2) , CGPathElementType :: AddCurveToPoint => slice :: from_raw_parts (self . points , 3) , } } } }
};
}
