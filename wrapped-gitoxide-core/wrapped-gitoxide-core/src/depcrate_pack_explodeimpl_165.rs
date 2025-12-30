// Generated macro for impl_165 (impl)
macro_rules! Depcrate_pack_explodeimpl_165 {
() => {
// Module: crate::pack::explode
// Provides: {"impl_165"}
// Dependencies: {}
impl gix :: objs :: Write for OutputWriter { fn write_buf (& self , kind : object :: Kind , from : & [u8]) -> Result < ObjectId , gix :: objs :: write :: Error > { match self { OutputWriter :: Loose (db) => db . write_buf (kind , from) , OutputWriter :: Sink (db) => db . write_buf (kind , from) , } } fn write_stream (& self , kind : object :: Kind , size : u64 , from : & mut dyn Read ,) -> Result < ObjectId , gix :: objs :: write :: Error > { match self { OutputWriter :: Loose (db) => db . write_stream (kind , size , from) , OutputWriter :: Sink (db) => db . write_stream (kind , size , from) , } } }
};
}
