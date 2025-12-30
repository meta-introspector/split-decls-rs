// Generated macro for impl_772 (impl)
macro_rules! Depcrate_tagged_ptrimpl_772 {
() => {
// Module: crate::tagged_ptr
// Provides: {"impl_772"}
// Dependencies: {}
impl < P , T > fmt :: Debug for TaggedRef < '_ , P , T > where P : Aligned + fmt :: Debug + ? Sized , T : Tag + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TaggedRef") . field ("pointer" , & self . pointer ()) . field ("tag" , & self . tag ()) . finish () } }
};
}
