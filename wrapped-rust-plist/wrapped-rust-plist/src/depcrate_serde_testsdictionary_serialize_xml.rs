// Generated macro for dictionary_serialize_xml (function)
macro_rules! Depcrate_serde_testsdictionary_serialize_xml {
() => {
// Module: crate::serde_tests
// Provides: {"dictionary_serialize_xml"}
// Dependencies: {}
# [test] fn dictionary_serialize_xml () { let mut inner_dict = Dictionary :: new () ; inner_dict . insert ("FirstKey" . to_owned () , Value :: String ("FirstValue" . to_owned ()) ,) ; inner_dict . insert ("SecondKey" . to_owned () , Value :: Data (vec ! [10 , 20 , 30 , 40])) ; inner_dict . insert ("ThirdKey" . to_owned () , Value :: Real (1.234)) ; inner_dict . insert ("FourthKey" . to_owned () , Value :: Date (Date :: from_xml_format ("1981-05-16T11:32:06Z") . unwrap ()) ,) ; let mut dict = Dictionary :: new () ; dict . insert ("AnArray" . to_owned () , Value :: Array (vec ! [Value :: String ("Hello, world!" . to_owned ()) , Value :: Integer (Integer :: from (345)) ,]) ,) ; dict . insert ("ADictionary" . to_owned () , Value :: Dictionary (inner_dict)) ; dict . insert ("AnInteger" . to_owned () , Value :: Integer (Integer :: from (123))) ; dict . insert ("ATrueBoolean" . to_owned () , Value :: Boolean (true)) ; dict . insert ("AFalseBoolean" . to_owned () , Value :: Boolean (false)) ; let mut buf = Cursor :: new (Vec :: new ()) ; crate :: to_writer_xml (& mut buf , & dict) . unwrap () ; let buf = buf . into_inner () ; let xml = std :: str :: from_utf8 (& buf) . unwrap () ; let comparison = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t<key>AnArray</key>
\t<array>
\t\t<string>Hello, world!</string>
\t\t<integer>345</integer>
\t</array>
\t<key>ADictionary</key>
\t<dict>
\t\t<key>FirstKey</key>
\t\t<string>FirstValue</string>
\t\t<key>SecondKey</key>
\t\t<data>
\t\tChQeKA==
\t\t</data>
\t\t<key>ThirdKey</key>
\t\t<real>1.234</real>
\t\t<key>FourthKey</key>
\t\t<date>1981-05-16T11:32:06Z</date>
\t</dict>
\t<key>AnInteger</key>
\t<integer>123</integer>
\t<key>ATrueBoolean</key>
\t<true/>
\t<key>AFalseBoolean</key>
\t<false/>
</dict>
</plist>" ; assert_eq ! (xml , comparison) ; }
};
}
