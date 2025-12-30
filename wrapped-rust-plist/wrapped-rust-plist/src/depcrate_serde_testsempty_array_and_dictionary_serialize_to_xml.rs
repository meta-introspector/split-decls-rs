// Generated macro for empty_array_and_dictionary_serialize_to_xml (function)
macro_rules! Depcrate_serde_testsempty_array_and_dictionary_serialize_to_xml {
() => {
// Module: crate::serde_tests
// Provides: {"empty_array_and_dictionary_serialize_to_xml"}
// Dependencies: {}
# [test] fn empty_array_and_dictionary_serialize_to_xml () { # [derive (Serialize , Default)] struct Empty { vec : Vec < String > , map : BTreeMap < String , String > , } let mut buf = Cursor :: new (Vec :: new ()) ; crate :: to_writer_xml (& mut buf , & Empty :: default ()) . unwrap () ; let buf = buf . into_inner () ; let xml = std :: str :: from_utf8 (& buf) . unwrap () ; let comparison = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t<key>vec</key>
\t<array/>
\t<key>map</key>
\t<dict/>
</dict>
</plist>" ; assert_eq ! (xml , comparison) ; }
};
}
