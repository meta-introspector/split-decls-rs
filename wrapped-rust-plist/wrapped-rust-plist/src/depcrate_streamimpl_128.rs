// Generated macro for impl_128 (impl)
macro_rules! Depcrate_streamimpl_128 {
() => {
// Module: crate::stream
// Provides: {"impl_128"}
// Dependencies: {}
impl < 'a > Iterator for Events < 'a > { type Item = Event < 'a > ; fn next (& mut self) -> Option < Event < 'a > > { fn handle_value < 'c , 'b : 'c > (value : & 'b Value , stack : & 'c mut Vec < StackItem < 'b > > ,) -> Event < 'b > { match value { Value :: Array (array) => { let len = array . len () ; let iter = array . iter () ; stack . push (StackItem :: Array (iter)) ; Event :: StartArray (Some (len as u64)) } Value :: Dictionary (dict) => { let len = dict . len () ; let iter = dict . into_iter () ; stack . push (StackItem :: Dict (iter)) ; Event :: StartDictionary (Some (len as u64)) } Value :: Boolean (value) => Event :: Boolean (* value) , Value :: Data (value) => Event :: Data (Cow :: Borrowed (value)) , Value :: Date (value) => Event :: Date (* value) , Value :: Real (value) => Event :: Real (* value) , Value :: Integer (value) => Event :: Integer (* value) , Value :: String (value) => Event :: String (Cow :: Borrowed (value . as_str ())) , Value :: Uid (value) => Event :: Uid (* value) , } } Some (match self . stack . pop () ? { StackItem :: Root (value) | StackItem :: DictValue (value) => handle_value (value , & mut self . stack) , StackItem :: Array (mut array) => { if let Some (value) = array . next () { self . stack . push (StackItem :: Array (array)) ; handle_value (value , & mut self . stack) } else { Event :: EndCollection } } StackItem :: Dict (mut dict) => { if let Some ((key , value)) = dict . next () { self . stack . push (StackItem :: Dict (dict)) ; self . stack . push (StackItem :: DictValue (value)) ; Event :: String (Cow :: Borrowed (key)) } else { Event :: EndCollection } } }) } }
};
}
