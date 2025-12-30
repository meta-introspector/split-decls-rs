// Generated macro for UnifiedDiffPrinter (trait)
macro_rules! Depcrate_unified_diffUnifiedDiffPrinter {
() => {
// Module: crate::unified_diff
// Provides: {"UnifiedDiffPrinter"}
// Dependencies: {}
pub trait UnifiedDiffPrinter { fn display_header (& self , f : impl fmt :: Write , start_before : u32 , start_after : u32 , len_before : u32 , len_after : u32 ,) -> fmt :: Result ; fn display_context_token (& self , f : impl fmt :: Write , token : Token) -> fmt :: Result ; fn display_hunk (& self , f : impl fmt :: Write , before : & [Token] , after : & [Token]) -> fmt :: Result ; }
};
}
