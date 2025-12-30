// Generated macro for TypeFreshener (struct)
macro_rules! Depcrate_infer_freshenTypeFreshener {
() => {
// Module: crate::infer::freshen
// Provides: {"TypeFreshener"}
// Dependencies: {}
pub struct TypeFreshener < 'a , 'tcx > { infcx : & 'a InferCtxt < 'tcx > , ty_freshen_count : u32 , const_freshen_count : u32 , ty_freshen_map : FxHashMap < ty :: InferTy , Ty < 'tcx > > , const_freshen_map : FxHashMap < ty :: InferConst , ty :: Const < 'tcx > > , }
};
}
