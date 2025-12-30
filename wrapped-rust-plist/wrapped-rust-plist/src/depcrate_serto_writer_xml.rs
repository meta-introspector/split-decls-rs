// Generated macro for to_writer_xml (function)
macro_rules! Depcrate_serto_writer_xml {
() => {
// Module: crate::ser
// Provides: {"to_writer_xml"}
// Dependencies: {}
# [doc = " Serializes the given data structure to a byte stream as an XML encoded plist."] pub fn to_writer_xml < W : Write , T : ser :: Serialize > (writer : W , value : & T) -> Result < () , Error > { to_writer_xml_with_options (writer , value , & XmlWriteOptions :: default ()) }
};
}
