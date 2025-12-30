// Generated macro for impl_87 (impl)
macro_rules! Depcrateimpl_87 {
() => {
// Module: crate
// Provides: {"impl_87"}
// Dependencies: {}
impl < I , F > DoubleEndedFallibleIterator for Filter < I , F > where I : DoubleEndedFallibleIterator , F : FnMut (& I :: Item) -> Result < bool , I :: Error > , { # [inline] fn next_back (& mut self) -> Result < Option < I :: Item > , I :: Error > { let filter = & mut self . f ; self . it . try_rfold (() , | () , v | { if filter (& v) ? { return Err (FoldStop :: Break (Some (v))) ; } Ok (()) }) . map (| () | None) . unpack_fold () } # [inline] fn try_rfold < B , E , G > (& mut self , init : B , mut f : G) -> Result < B , E > where E : From < I :: Error > , G : FnMut (B , I :: Item) -> Result < B , E > , { let predicate = & mut self . f ; self . it . try_rfold (init , | acc , v | { if predicate (& v) ? { f (acc , v) } else { Ok (acc) } } ,) } }
};
}
