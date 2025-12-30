// Generated macro for impl_2004 (impl)
macro_rules! Depcrate_serializeimpl_2004 {
() => {
// Module: crate::serialize
// Provides: {"impl_2004"}
// Dependencies: {}
impl < 'a , 'b , DB : Backend > Output < 'a , 'b , DB > { # [doc = " Construct a new `Output`"] pub fn new (out : < DB :: BindCollector < 'a > as BindCollector < 'a , DB > > :: Buffer , metadata_lookup : & 'b mut DB :: MetadataLookup ,) -> Self { Output { out , metadata_lookup : Some (metadata_lookup) , } } # [doc = " Consume the current `Output` structure to access the inner buffer type"] # [doc = ""] # [doc = " This function is only useful for people implementing their own Backend."] pub fn into_inner (self) -> < DB :: BindCollector < 'a > as BindCollector < 'a , DB > > :: Buffer { self . out } # [doc = " Returns the backend's mechanism for dynamically looking up type"] # [doc = " metadata at runtime, if relevant for the given backend."] pub fn metadata_lookup (& mut self) -> & mut DB :: MetadataLookup { self . metadata_lookup . as_mut () . expect ("Lookup is there") } # [doc = " Set the inner buffer to a specific value"] # [doc = ""] # [doc = " Checkout the documentation of the type of `BindCollector::Buffer`"] # [doc = " for your specific backend for supported types."] pub fn set_value < V > (& mut self , value : V) where V : Into < < DB :: BindCollector < 'a > as BindCollector < 'a , DB > > :: Buffer > , { self . out = value . into () ; } }
};
}
