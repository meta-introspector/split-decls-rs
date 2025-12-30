// Generated macro for impl_73 (impl)
macro_rules! Depcrate_stream_binary_writerimpl_73 {
() => {
// Module: crate::stream::binary_writer
// Provides: {"impl_73"}
// Dependencies: {}
impl Value < '_ > { fn into_owned (self) -> Value < 'static > { match self { Value :: Boolean (v) => Value :: Boolean (v) , Value :: Data (v) => Value :: Data (Cow :: Owned (v . into_owned ())) , Value :: Date (v) => Value :: Date (v) , Value :: Integer (v) => Value :: Integer (v) , Value :: Real (v) => Value :: Real (v) , Value :: String (v) => Value :: String (Cow :: Owned (v . into_owned ())) , Value :: Uid (v) => Value :: Uid (v) , } } fn event_kind (& self) -> EventKind { match self { Value :: Boolean (_) => EventKind :: Boolean , Value :: Data (_) => EventKind :: Data , Value :: Date (_) => EventKind :: Date , Value :: Integer (_) => EventKind :: Integer , Value :: Real (_) => EventKind :: Real , Value :: String (_) => EventKind :: String , Value :: Uid (_) => EventKind :: Uid , } } }
};
}
