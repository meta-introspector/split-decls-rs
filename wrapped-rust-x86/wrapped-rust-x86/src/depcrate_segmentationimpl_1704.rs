// Generated macro for impl_1704 (impl)
macro_rules! Depcrate_segmentationimpl_1704 {
() => {
// Module: crate::segmentation
// Provides: {"impl_1704"}
// Dependencies: {}
impl fmt :: Display for SegmentSelector { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let r0 = match self . contains (SegmentSelector :: RPL_0) { false => "" , true => "Ring 0 segment selector." , } ; let r1 = match self . contains (SegmentSelector :: RPL_1) { false => "" , true => "Ring 1 segment selector." , } ; let r2 = match self . contains (SegmentSelector :: RPL_2) { false => "" , true => "Ring 2 segment selector." , } ; let r3 = match self . contains (SegmentSelector :: RPL_3) { false => "" , true => "Ring 3 segment selector." , } ; let tbl = match self . contains (SegmentSelector :: TI_LDT) { false => "GDT Table" , true => "LDT Table" , } ; write ! (f , "Index {} in {}, {}{}{}{}" , self . bits >> 3 , tbl , r0 , r1 , r2 , r3) } }
};
}
