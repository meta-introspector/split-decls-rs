// Generated macro for to_writer_xml_with_options (function)
macro_rules! Depcrate_serto_writer_xml_with_options {
() => {
// Module: crate::ser
// Provides: {"to_writer_xml_with_options"}
// Dependencies: {}
# [doc = " Serializes to a byte stream as an XML encoded plist, using custom [`XmlWriteOptions`]."] pub fn to_writer_xml_with_options < W : Write , T : ser :: Serialize > (writer : W , value : & T , options : & XmlWriteOptions ,) -> Result < () , Error > { let writer = stream :: XmlWriter :: new_with_options (writer , options) ; let mut ser = Serializer :: new (writer) ; value . serialize (& mut ser) }
};
}
