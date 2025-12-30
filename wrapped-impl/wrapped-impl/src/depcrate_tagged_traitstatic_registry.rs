// Generated macro for static_registry (function)
macro_rules! Depcrate_tagged_traitstatic_registry {
() => {
// Module: crate::tagged_trait
// Provides: {"static_registry"}
// Dependencies: {}
fn static_registry () -> TokenStream { quote ! { static TYPETAG : typetag ::# private :: once_cell :: race :: OnceBox < typetag ::# private :: Registry < TypetagStrictest >> = typetag ::# private :: once_cell :: race :: OnceBox :: new () ; let registry = TYPETAG . get_or_init (|| { let mut map = typetag ::# private :: BTreeMap :: new () ; let mut names = typetag ::# private :: Vec :: new () ; for registered in typetag ::# private :: inventory :: iter ::< TypetagRegistration < TypetagFn >> { match map . entry (registered . name) { typetag ::# private :: btree_map :: Entry :: Vacant (entry) => { entry . insert (typetag ::# private :: Option :: Some (registered . deserializer)) ; } typetag ::# private :: btree_map :: Entry :: Occupied (mut entry) => { entry . insert (typetag ::# private :: Option :: None) ; } } names . push (registered . name) ; } names . sort_unstable () ; typetag ::# private :: Box :: new (typetag ::# private :: Registry { map , names }) }) ; } }
};
}
