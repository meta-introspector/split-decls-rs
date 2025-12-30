// Generated macro for impl_249 (impl)
macro_rules! Depcrate_valueimpl_249 {
() => {
// Module: crate::value
// Provides: {"impl_249"}
// Dependencies: {}
impl Builder { fn build < 'event , T > (stream : T) -> Result < Value , Error > where T : Iterator < Item = Result < Event < 'event > , Error > > , { let mut builder = Self :: default () ; for event in stream { builder . write (event ?) ? ; } builder . finish () } fn write_value (& mut self , value : Value) -> Result < () , Error > { match (self . stack . pop () , value) { (None , value) => self . stack . push (StackItem :: Root (value)) , (Some (StackItem :: Root (_)) , value) => { return Err (ErrorKind :: ExpectedEndOfEventStream { found : EventKind :: of_value (& value) , } . without_position ()) } (Some (StackItem :: Array (mut array)) , value) => { array . push (value) ; self . stack . push (StackItem :: Array (array)) ; } (Some (StackItem :: Dict (dict)) , Value :: String (key)) => { self . stack . push (StackItem :: DictAndKey (dict , key)) ; } (Some (StackItem :: Dict (_)) , value) => { return Err (ErrorKind :: UnexpectedEventType { expected : EventKind :: DictionaryKeyOrEndCollection , found : EventKind :: of_value (& value) , } . without_position ()) } (Some (StackItem :: DictAndKey (mut dict , key)) , value) => { dict . insert (key , value) ; self . stack . push (StackItem :: Dict (dict)) ; } } Ok (()) } pub fn finish (& mut self) -> Result < Value , Error > { match self . stack . pop () { Some (StackItem :: Root (value)) => Ok (value) , _ => Err (ErrorKind :: UnexpectedEndOfEventStream . without_position ()) , } } }
};
}
