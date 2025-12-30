// Generated macro for impl_141 (impl)
macro_rules! Depcrate_manifestimpl_141 {
() => {
// Module: crate::manifest
// Provides: {"impl_141"}
// Dependencies: {}
impl < T > InheritableField < T > { pub fn normalized (& self) -> Result < & T , UnresolvedError > { self . as_value () . ok_or (UnresolvedError) } pub fn as_value (& self) -> Option < & T > { match self { InheritableField :: Inherit (_) => None , InheritableField :: Value (defined) => Some (defined) , } } pub fn into_value (self) -> Option < T > { match self { Self :: Inherit (_) => None , Self :: Value (defined) => Some (defined) , } } pub fn is_inherited (& self) -> bool { matches ! (self , Self :: Inherit (_)) } }
};
}
