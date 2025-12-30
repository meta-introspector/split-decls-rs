// Generated macro for impl_130 (impl)
macro_rules! Depcrate_ext_deimpl_130 {
() => {
// Module: crate::ext::de
// Provides: {"impl_130"}
// Dependencies: {}
impl < 'de > ValueBase < 'de > for ValueRef < 'de > { type Item = Self ; type Iter = IntoIter < Self > ; type MapIter = IntoIter < (Self , Self) > ; type MapDeserializer = MapDeserializer < Self :: MapIter , Self :: Item > ; # [inline] fn is_nil (& self) -> bool { * self == Self :: Nil } # [inline] fn into_iter (self) -> Result < Self :: Iter , Self :: Item > { match self { Self :: Array (v) => Ok (v . into_iter ()) , other => Err (other) , } } # [inline] fn into_map_iter (self) -> Result < Self :: MapIter , Self :: Item > { match self { Self :: Map (v) => Ok (v . into_iter ()) , other => Err (other) , } } }
};
}
