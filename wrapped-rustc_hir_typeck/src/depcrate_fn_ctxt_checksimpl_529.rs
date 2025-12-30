// Generated macro for impl_529 (impl)
macro_rules! Depcrate_fn_ctxt_checksimpl_529 {
() => {
// Module: crate::fn_ctxt::checks
// Provides: {"impl_529"}
// Dependencies: {}
impl FnParam < '_ > { fn span (& self) -> Span { match self { Self :: Param (param) => param . span , Self :: Ident (ident) => { if let Some (ident) = ident { ident . span } else { DUMMY_SP } } } } fn display (& self , idx : usize) -> impl '_ + fmt :: Display { struct D < 'a > (FnParam < 'a > , usize) ; impl fmt :: Display for D < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let unique_name = match self . 0 { FnParam :: Param (param) if let hir :: PatKind :: Binding (_ , _ , ident , _) = param . pat . kind => { Some (ident . name) } FnParam :: Ident (ident) if let Some (ident) = ident && ident . name != kw :: Underscore => { Some (ident . name) } _ => None , } ; if let Some (unique_name) = unique_name { write ! (f , "`{unique_name}`") } else { write ! (f , "parameter #{}" , self . 1 + 1) } } } D (* self , idx) } }
};
}
