// Generated macro for impl_187 (impl)
macro_rules! Depcrate_errorimpl_187 {
() => {
// Module: crate::error
// Provides: {"impl_187"}
// Dependencies: {}
impl EventKind { # [cfg (feature = "serde")] pub fn of_event (event : & Event) -> EventKind { match event { Event :: StartArray (_) => EventKind :: StartArray , Event :: StartDictionary (_) => EventKind :: StartDictionary , Event :: EndCollection => EventKind :: EndCollection , Event :: Boolean (_) => EventKind :: Boolean , Event :: Data (_) => EventKind :: Data , Event :: Date (_) => EventKind :: Date , Event :: Integer (_) => EventKind :: Integer , Event :: Real (_) => EventKind :: Real , Event :: String (_) => EventKind :: String , Event :: Uid (_) => EventKind :: Uid , } } pub fn of_value (event : & Value) -> EventKind { match event { Value :: Array (_) => EventKind :: StartArray , Value :: Dictionary (_) => EventKind :: StartDictionary , Value :: Boolean (_) => EventKind :: Boolean , Value :: Data (_) => EventKind :: Data , Value :: Date (_) => EventKind :: Date , Value :: Integer (_) => EventKind :: Integer , Value :: Real (_) => EventKind :: Real , Value :: String (_) => EventKind :: String , Value :: Uid (_) => EventKind :: Uid , } } }
};
}
