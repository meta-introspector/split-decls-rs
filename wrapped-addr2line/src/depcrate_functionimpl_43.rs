// Generated macro for impl_43 (impl)
macro_rules! Depcrate_functionimpl_43 {
() => {
// Module: crate::function
// Provides: {"impl_43"}
// Dependencies: {}
impl < R : gimli :: Reader > LazyFunction < R > { fn new (dw_die_offset : gimli :: UnitOffset < R :: Offset >) -> Self { LazyFunction { dw_die_offset , lazy : LazyResult :: new () , } } pub (crate) fn borrow (& self , file : DebugFile , unit : gimli :: UnitRef < R > , ctx : & Context < R > ,) -> Result < & Function < R > , Error > { self . lazy . get_or_init (| | Function :: parse (self . dw_die_offset , file , unit , ctx)) . as_ref () . map_err (Error :: clone) } }
};
}
