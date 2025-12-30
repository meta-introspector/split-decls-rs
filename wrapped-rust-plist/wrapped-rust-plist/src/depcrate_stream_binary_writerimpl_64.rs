// Generated macro for impl_64 (impl)
macro_rules! Depcrate_stream_binary_writerimpl_64 {
() => {
// Module: crate::stream::binary_writer
// Provides: {"impl_64"}
// Dependencies: {}
impl < W : Write > Writer for BinaryWriter < W > { fn write_start_array (& mut self , _len : Option < u64 >) -> Result < () , Error > { self . write_start_collection (CollectionType :: Array) } fn write_start_dictionary (& mut self , _len : Option < u64 >) -> Result < () , Error > { self . write_start_collection (CollectionType :: Dictionary) } fn write_end_collection (& mut self) -> Result < () , Error > { self . write_end_collection () } fn write_boolean (& mut self , value : bool) -> Result < () , Error > { self . write_value (Value :: Boolean (value)) } fn write_data (& mut self , value : Cow < [u8] >) -> Result < () , Error > { self . write_value (Value :: Data (value)) } fn write_date (& mut self , value : Date) -> Result < () , Error > { self . write_value (Value :: Date (value)) } fn write_integer (& mut self , value : Integer) -> Result < () , Error > { self . write_value (Value :: Integer (value)) } fn write_real (& mut self , value : f64) -> Result < () , Error > { self . write_value (Value :: Real (value . to_bits ())) } fn write_string (& mut self , value : Cow < str >) -> Result < () , Error > { self . write_value (Value :: String (value)) } fn write_uid (& mut self , value : Uid) -> Result < () , Error > { self . write_value (Value :: Uid (value)) } }
};
}
