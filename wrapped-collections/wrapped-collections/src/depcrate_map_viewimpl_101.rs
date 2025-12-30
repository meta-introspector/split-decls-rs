// Generated macro for impl_101 (impl)
macro_rules! Depcrate_map_viewimpl_101 {
() => {
// Module: crate::map_view
// Provides: {"impl_101"}
// Dependencies: {}
impl < K , V > IIterable_Impl < IKeyValuePair < K , V > > for StockMapView_Impl < K , V > where K : RuntimeType , V : RuntimeType , K :: Default : Clone + Ord , V :: Default : Clone , { fn First (& self) -> Result < IIterator < IKeyValuePair < K , V > > > { Ok (ComObject :: new (StockMapViewIterator :: < K , V > { _owner : self . to_object () , current : std :: sync :: RwLock :: new (self . map . iter ()) , }) . into_interface ()) } }
};
}
