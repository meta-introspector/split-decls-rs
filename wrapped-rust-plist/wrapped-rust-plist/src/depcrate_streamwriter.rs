// Generated macro for Writer (trait)
macro_rules! Depcrate_streamWriter {
() => {
// Module: crate::stream
// Provides: {"Writer"}
// Dependencies: {}
# [doc = " Supports writing event streams in different plist encodings."] pub trait Writer : private :: Sealed { fn write (& mut self , event : Event) -> Result < () , Error > { match event { Event :: StartArray (len) => self . write_start_array (len) , Event :: StartDictionary (len) => self . write_start_dictionary (len) , Event :: EndCollection => self . write_end_collection () , Event :: Boolean (value) => self . write_boolean (value) , Event :: Data (value) => self . write_data (value) , Event :: Date (value) => self . write_date (value) , Event :: Integer (value) => self . write_integer (value) , Event :: Real (value) => self . write_real (value) , Event :: String (value) => self . write_string (value) , Event :: Uid (value) => self . write_uid (value) , } } fn write_start_array (& mut self , len : Option < u64 >) -> Result < () , Error > ; fn write_start_dictionary (& mut self , len : Option < u64 >) -> Result < () , Error > ; fn write_end_collection (& mut self) -> Result < () , Error > ; fn write_boolean (& mut self , value : bool) -> Result < () , Error > ; fn write_data (& mut self , value : Cow < [u8] >) -> Result < () , Error > ; fn write_date (& mut self , value : Date) -> Result < () , Error > ; fn write_integer (& mut self , value : Integer) -> Result < () , Error > ; fn write_real (& mut self , value : f64) -> Result < () , Error > ; fn write_string (& mut self , value : Cow < str >) -> Result < () , Error > ; fn write_uid (& mut self , value : Uid) -> Result < () , Error > ; }
};
}
