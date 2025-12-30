// Generated macro for UninhabitedFrom (struct)
macro_rules! Depcrate_inhabitednessUninhabitedFrom {
() => {
// Module: crate::inhabitedness
// Provides: {"UninhabitedFrom"}
// Dependencies: {}
struct UninhabitedFrom < 'a , 'db > { target_mod : ModuleId , recursive_ty : FxHashSet < Ty < 'db > > , max_depth : usize , infcx : & 'a InferCtxt < 'db > , env : Arc < TraitEnvironment < 'db > > , }
};
}
