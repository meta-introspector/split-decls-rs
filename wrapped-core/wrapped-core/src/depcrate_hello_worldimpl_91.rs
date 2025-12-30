// Generated macro for impl_91 (impl)
macro_rules! Depcrate_hello_worldimpl_91 {
() => {
// Module: crate::hello_world
// Provides: {"impl_91"}
// Dependencies: {}
impl IterableDataProvider < HelloWorldV1 > for HelloWorldProvider { fn iter_ids (& self) -> Result < BTreeSet < DataIdentifierCow < '_ > > , DataError > { # [expect (clippy :: unwrap_used)] Ok (Self :: DATA . iter () . map (| (l , a , _) | { DataIdentifierCow :: from_borrowed_and_owned (DataMarkerAttributes :: from_str_or_panic (a) , l . parse () . unwrap () ,) }) . collect ()) } }
};
}
