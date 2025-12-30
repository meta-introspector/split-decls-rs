// Generated macro for plist_value_display (function)
macro_rules! Depcrate_fs_feature_xattrplist_value_display {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"plist_value_display"}
// Dependencies: {}
fn plist_value_display (value : & [u8]) -> Option < String > { let reader = io :: Cursor :: new (value) ; plist :: Value :: from_reader (reader) . ok () . and_then (| v | { let mut buffer = Vec :: new () ; v . to_writer_xml_with_options (BorrowedWriter { buffer : & mut buffer , } , & plist :: XmlWriteOptions :: default () . indent (b' ' , 0) . root_element (false) ,) . ok () . and_then (| () | str :: from_utf8 (& buffer) . ok ()) . map (| s | format ! ("<plist version=\"1.0\">{}</plist>" , s . replace ('\n' , ""))) }) }
};
}
