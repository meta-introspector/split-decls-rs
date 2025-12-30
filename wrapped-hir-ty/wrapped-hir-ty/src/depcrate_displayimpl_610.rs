// Generated macro for impl_610 (impl)
macro_rules! Depcrate_displayimpl_610 {
() => {
// Module: crate::display
// Provides: {"impl_610"}
// Dependencies: {}
impl HirDisplay for Const { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { let data = self . interned () ; match & data . value { ConstValue :: BoundVar (idx) => idx . hir_fmt (f) , ConstValue :: InferenceVar (..) => write ! (f , "#c#") , ConstValue :: Placeholder (idx) => { let id = from_placeholder_idx (f . db , * idx) ; let generics = generics (f . db , id . parent) ; let param_data = & generics [id . local_id] ; write ! (f , "{}" , param_data . name () . unwrap () . display (f . db , f . edition ())) ? ; Ok (()) } ConstValue :: Concrete (c) => match & c . interned { ConstScalar :: Bytes (b , m) => render_const_scalar (f , b , m , & data . ty) , ConstScalar :: UnevaluatedConst (c , parameters) => { write ! (f , "{}" , c . name (f . db)) ? ; hir_fmt_generics (f , parameters . as_slice (Interner) , c . generic_def (f . db) , None) ? ; Ok (()) } ConstScalar :: Unknown => f . write_char ('_') , } , } } }
};
}
