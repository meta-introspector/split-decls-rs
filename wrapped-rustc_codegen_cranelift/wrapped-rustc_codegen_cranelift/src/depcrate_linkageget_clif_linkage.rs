// Generated macro for get_clif_linkage (function)
macro_rules! Depcrate_linkageget_clif_linkage {
() => {
// Module: crate::linkage
// Provides: {"get_clif_linkage"}
// Dependencies: {}
pub (crate) fn get_clif_linkage (mono_item : MonoItem < '_ > , linkage : RLinkage , visibility : Visibility , is_compiler_builtins : bool ,) -> Linkage { match (linkage , visibility) { (RLinkage :: External , Visibility :: Default) if is_compiler_builtins => Linkage :: Hidden , (RLinkage :: External , Visibility :: Default) => Linkage :: Export , (RLinkage :: Internal , Visibility :: Default) => Linkage :: Local , (RLinkage :: External , Visibility :: Hidden) => Linkage :: Hidden , (RLinkage :: WeakAny , Visibility :: Default) => Linkage :: Preemptible , _ => panic ! ("{:?} = {:?} {:?}" , mono_item , linkage , visibility) , } }
};
}
