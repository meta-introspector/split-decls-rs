// Generated macro for impl_324 (impl)
macro_rules! Depcrate_serde_testsimpl_324 {
() => {
// Module: crate::serde_tests
// Provides: {"impl_324"}
// Dependencies: {}
impl Writer for VecWriter { fn write_start_array (& mut self , len : Option < u64 >) -> Result < () , Error > { self . events . push (Event :: StartArray (len)) ; Ok (()) } fn write_start_dictionary (& mut self , len : Option < u64 >) -> Result < () , Error > { self . events . push (Event :: StartDictionary (len)) ; Ok (()) } fn write_end_collection (& mut self) -> Result < () , Error > { self . events . push (Event :: EndCollection) ; Ok (()) } fn write_boolean (& mut self , value : bool) -> Result < () , Error > { self . events . push (Event :: Boolean (value)) ; Ok (()) } fn write_data (& mut self , value : Cow < [u8] >) -> Result < () , Error > { self . events . push (Event :: Data (Cow :: Owned (value . into_owned ()))) ; Ok (()) } fn write_date (& mut self , value : Date) -> Result < () , Error > { self . events . push (Event :: Date (value)) ; Ok (()) } fn write_integer (& mut self , value : Integer) -> Result < () , Error > { self . events . push (Event :: Integer (value)) ; Ok (()) } fn write_real (& mut self , value : f64) -> Result < () , Error > { self . events . push (Event :: Real (value)) ; Ok (()) } fn write_string (& mut self , value : Cow < str >) -> Result < () , Error > { self . events . push (Event :: String (Cow :: Owned (value . into_owned ()))) ; Ok (()) } fn write_uid (& mut self , value : Uid) -> Result < () , Error > { self . events . push (Event :: Uid (value)) ; Ok (()) } }
};
}
