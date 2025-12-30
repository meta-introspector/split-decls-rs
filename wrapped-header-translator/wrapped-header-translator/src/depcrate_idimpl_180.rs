// Generated macro for impl_180 (impl)
macro_rules! Depcrate_idimpl_180 {
() => {
// Module: crate::id
// Provides: {"impl_180"}
// Dependencies: {}
impl ItemIdentifier < Option < String > > { pub fn new_optional (entity : & Entity < '_ > , context : & Context < '_ >) -> Self { let name = if ! entity . is_anonymous () { entity . get_name () } else { None } ; Self :: with_name (name , entity , context) } pub fn to_option (self) -> Option < ItemIdentifier > { if let Some (name) = self . name { Some (ItemIdentifier :: from_raw (name , self . location)) } else { None } } # [track_caller] pub fn require_name (self) -> ItemIdentifier { self . to_option () . expect ("item must name a name") } }
};
}
