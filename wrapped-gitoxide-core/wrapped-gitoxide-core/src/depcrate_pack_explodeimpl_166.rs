// Generated macro for impl_166 (impl)
macro_rules! Depcrate_pack_explodeimpl_166 {
() => {
// Module: crate::pack::explode
// Provides: {"impl_166"}
// Dependencies: {}
impl OutputWriter { fn new (path : Option < impl AsRef < Path > > , compress : bool , object_hash : gix :: hash :: Kind) -> Self { match path { Some (path) => OutputWriter :: Loose (loose :: Store :: at (path . as_ref () , object_hash)) , None => OutputWriter :: Sink (odb :: sink (object_hash) . compress (compress)) , } } }
};
}
