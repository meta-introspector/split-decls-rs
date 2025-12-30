// Generated macro for impl_69 (impl)
macro_rules! Depcrate_locationimpl_69 {
() => {
// Module: crate::location
// Provides: {"impl_69"}
// Dependencies: {}
impl Location { # [doc = " Returns a unique id of current location."] # [doc = ""] # [doc = " Returns [`None`] if current location is not created by `gloo::history`."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " Depending on the situation, the id may or may not be sequential / incremental."] pub fn id (& self) -> Option < u32 > { self . id } # [doc = " Returns the `pathname` of current location."] pub fn path (& self) -> & str { & self . path } # [doc = " Returns the queries of current URL in [`&str`]."] pub fn query_str (& self) -> & str { & self . query_str } # [doc = " Returns the queries of current URL parsed as `T`."] # [cfg (feature = "query")] pub fn query < T > (& self) -> HistoryResult < T :: Target , T :: Error > where T : FromQuery , { let query = self . query_str () . strip_prefix ('?') . unwrap_or ("") ; T :: from_query (query) } # [doc = " Returns the hash fragment of current URL."] pub fn hash (& self) -> & str { & self . hash } # [doc = " Returns an Rc'ed state of current location."] # [doc = ""] # [doc = " Returns [`None`] if state is not created by `gloo::history`, or state fails to downcast."] pub fn state < T > (& self) -> Option < Rc < T > > where T : 'static , { self . state . clone () . and_then (| m | m . downcast () . ok ()) } }
};
}
