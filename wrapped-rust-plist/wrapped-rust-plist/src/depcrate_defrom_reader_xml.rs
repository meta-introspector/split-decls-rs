// Generated macro for from_reader_xml (function)
macro_rules! Depcrate_defrom_reader_xml {
() => {
// Module: crate::de
// Provides: {"from_reader_xml"}
// Dependencies: {}
# [doc = " Deserializes an instance of type `T` from a byte stream containing an XML encoded plist."] pub fn from_reader_xml < R : Read , T : de :: DeserializeOwned > (reader : R) -> Result < T , Error > { let reader = stream :: XmlReader :: new (BufReader :: new (reader)) ; from_stream (reader) }
};
}
