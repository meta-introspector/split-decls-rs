// Generated macro for impl_397 (impl)
macro_rules! Depcrate_stmtimpl_397 {
() => {
// Module: crate::stmt
// Provides: {"impl_397"}
// Dependencies: {}
impl < 'a , I : IntoIterator < Item = & 'a GenericWithBound > + Clone > fmt :: Display for GenericParamsHelper < 'a , I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut iter = self . 0 . clone () . into_iter () ; if let Some ((first , bound)) = iter . next () { write ! (f , "<{first}: {}" , self . 1) ? ; if let Some (bound) = bound { write ! (f , " + {}" , bound . generic_bound ()) ? ; } for (generic , bound) in iter { write ! (f , ", {generic}: {}" , self . 1) ? ; if let Some (bound) = bound { write ! (f , " + {}" , bound . generic_bound ()) ? ; } } write ! (f , ">") ? ; } Ok (()) } }
};
}
