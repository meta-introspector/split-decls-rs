// Generated macro for impl_526 (impl)
macro_rules! Depcrate_pad_tailimpl_526 {
() => {
// Module: crate::pad_tail
// Provides: {"impl_526"}
// Dependencies: {}
impl < I , F > DoubleEndedIterator for PadUsing < I , F > where I : DoubleEndedIterator + ExactSizeIterator , F : FnMut (usize) -> I :: Item , { fn next_back (& mut self) -> Option < Self :: Item > { if self . min == 0 { self . iter . next_back () } else if self . iter . len () >= self . min { self . min -= 1 ; self . iter . next_back () } else { self . min -= 1 ; Some ((self . filler) (self . min)) } } fn rfold < B , G > (self , mut init : B , mut f : G) -> B where G : FnMut (B , Self :: Item) -> B , { init = (self . iter . len () .. self . min) . map (self . filler) . rfold (init , & mut f) ; self . iter . rfold (init , f) } }
};
}
