// Generated macro for impl_129 (impl)
macro_rules! Depcrate_ext_deimpl_129 {
() => {
// Module: crate::ext::de
// Provides: {"impl_129"}
// Dependencies: {}
impl ValueBase < '_ > for Value { type Item = Self ; type Iter = IntoIter < Self > ; type MapIter = IntoIter < (Self , Self) > ; type MapDeserializer = MapDeserializer < Self :: MapIter , Self :: Item > ; # [inline] fn is_nil (& self) -> bool { * self == Self :: Nil } # [inline] fn into_iter (self) -> Result < Self :: Iter , Self :: Item > { match self { Self :: Array (v) => Ok (v . into_iter ()) , other => Err (other) , } } # [inline] fn into_map_iter (self) -> Result < Self :: MapIter , Self :: Item > { match self { Self :: Map (v) => Ok (v . into_iter ()) , other => Err (other) , } } }
};
}
