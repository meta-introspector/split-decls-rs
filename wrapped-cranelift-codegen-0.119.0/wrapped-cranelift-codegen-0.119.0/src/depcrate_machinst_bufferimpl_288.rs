// Generated macro for impl_288 (impl)
macro_rules! Depcrate_machinst_bufferimpl_288 {
() => {
// Module: crate::machinst::buffer
// Provides: {"impl_288"}
// Dependencies: {}
impl FinalizedRelocTarget { # [doc = " Returns a display for the current [FinalizedRelocTarget], with extra context to prettify the"] # [doc = " output."] pub fn display < 'a > (& 'a self , params : Option < & 'a FunctionParameters >) -> String { match self { FinalizedRelocTarget :: ExternalName (name) => format ! ("{}" , name . display (params)) , FinalizedRelocTarget :: Func (offset) => format ! ("func+{offset}") , } } }
};
}
