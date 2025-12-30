// Generated macro for impl_443 (impl)
macro_rules! Depcrate_schema_modelimpl_443 {
() => {
// Module: crate::schema::model
// Provides: {"impl_443"}
// Dependencies: {}
impl < 'a , S > TypeType < 'a , S > { pub fn to_concrete (& self) -> Option < & 'a MetaType < S > > { match self { Self :: Concrete (t) => Some (t) , Self :: List (..) | Self :: NonNull (..) => None , } } pub fn innermost_concrete (& self) -> & 'a MetaType < S > { match self { Self :: Concrete (t) => t , Self :: NonNull (n) | Self :: List (n , ..) => n . innermost_concrete () , } } pub fn list_contents (& self) -> Option < & Self > { match self { Self :: List (n , ..) => Some (n) , Self :: NonNull (n) => n . list_contents () , Self :: Concrete (..) => None , } } pub fn is_non_null (& self) -> bool { matches ! (self , TypeType :: NonNull (..)) } }
};
}
