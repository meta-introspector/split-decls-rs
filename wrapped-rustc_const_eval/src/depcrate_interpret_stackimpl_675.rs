// Generated macro for impl_675 (impl)
macro_rules! Depcrate_interpret_stackimpl_675 {
() => {
// Module: crate::interpret::stack
// Provides: {"impl_675"}
// Dependencies: {}
impl < 'tcx > FrameInfo < 'tcx > { pub fn as_note (& self , tcx : TyCtxt < 'tcx >) -> errors :: FrameNote { let span = self . span ; if tcx . def_key (self . instance . def_id ()) . disambiguated_data . data == DefPathData :: Closure { errors :: FrameNote { where_ : "closure" , span , instance : String :: new () , times : 0 , has_label : false , } } else { let instance = format ! ("{}" , self . instance) ; errors :: FrameNote { where_ : "instance" , span , instance , times : 0 , has_label : false } } } }
};
}
