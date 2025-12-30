// Generated macro for impl_892 (impl)
macro_rules! Depcrate_util_any_valueimpl_892 {
() => {
// Module: crate::util::any_value
// Provides: {"impl_892"}
// Dependencies: {}
impl AnyValue { pub (crate) fn new < V : std :: any :: Any + Clone + Send + Sync + 'static > (inner : V) -> Self { let id = AnyValueId :: of :: < V > () ; let inner = std :: sync :: Arc :: new (inner) ; Self { inner , id } } pub (crate) fn downcast_ref < T : std :: any :: Any + Clone + Send + Sync + 'static > (& self ,) -> Option < & T > { self . inner . downcast_ref :: < T > () } pub (crate) fn downcast_into < T : std :: any :: Any + Clone + Send + Sync > (self) -> Result < T , Self > { let id = self . id ; let value = ok ! (std :: sync :: Arc :: downcast ::< T > (self . inner) . map_err (| inner | Self { inner , id })) ; let value = std :: sync :: Arc :: try_unwrap (value) . unwrap_or_else (| arc | (* arc) . clone ()) ; Ok (value) } pub (crate) fn type_id (& self) -> AnyValueId { self . id } }
};
}
