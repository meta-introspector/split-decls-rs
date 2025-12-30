// Generated macro for impl_144 (impl)
macro_rules! Depcrate_value_deimpl_144 {
() => {
// Module: crate::value::de
// Provides: {"impl_144"}
// Dependencies: {}
impl < 'a > From < & 'a Value > for de :: Unexpected < 'a > { # [inline] fn from (value : & 'a Value) -> Self { match value { Value :: Bool (x) => Self :: Bool (* x) , Value :: Integer (x) => Self :: from (* x) , Value :: Float (x) => Self :: Float (* x) , Value :: Bytes (x) => Self :: Bytes (x) , Value :: Text (x) => Self :: Str (x) , Value :: Array (..) => Self :: Seq , Value :: Map (..) => Self :: Map , Value :: Null => Self :: Other ("null") , Value :: Tag (..) => Self :: Other ("tag") , } } }
};
}
