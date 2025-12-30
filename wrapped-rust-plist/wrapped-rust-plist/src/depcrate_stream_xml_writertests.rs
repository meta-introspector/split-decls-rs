// Generated macro for tests (module)
macro_rules! Depcrate_stream_xml_writertests {
() => {
// Module: crate::stream::xml_writer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: io :: Cursor ; use super :: * ; use crate :: stream :: Event ; # [test] fn streaming_parser () { let plist = [Event :: StartDictionary (None) , Event :: String ("Author" . into ()) , Event :: String ("William Shakespeare" . into ()) , Event :: String ("Lines" . into ()) , Event :: StartArray (None) , Event :: String ("It is a tale told by an idiot," . into ()) , Event :: String ("Full of sound and fury, signifying nothing." . into ()) , Event :: Data ((0 .. 128) . collect :: < Vec < _ > > () . into ()) , Event :: EndCollection , Event :: String ("Death" . into ()) , Event :: Integer (1564 . into ()) , Event :: String ("Height" . into ()) , Event :: Real (1.60) , Event :: String ("Data" . into ()) , Event :: Data (vec ! [0 , 0 , 0 , 190 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 30 , 0 , 0 , 0] . into ()) , Event :: String ("Birthdate" . into ()) , Event :: Date (super :: Date :: from_xml_format ("1981-05-16T11:32:06Z") . unwrap ()) , Event :: String ("Comment" . into ()) , Event :: String ("2 < 3" . into ()) , Event :: String ("BiggestNumber" . into ()) , Event :: Integer (18446744073709551615u64 . into ()) , Event :: String ("SmallestNumber" . into ()) , Event :: Integer ((- 9223372036854775808i64) . into ()) , Event :: String ("IsTrue" . into ()) , Event :: Boolean (true) , Event :: String ("IsNotFalse" . into ()) , Event :: Boolean (false) , Event :: EndCollection ,] ; let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t<key>Author</key>
\t<string>William Shakespeare</string>
\t<key>Lines</key>
\t<array>
\t\t<string>It is a tale told by an idiot,</string>
\t\t<string>Full of sound and fury, signifying nothing.</string>
\t\t<data>
\t\tAAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8gISIjJCUmJygpKissLS4vMDEy
\t\tMzQ1Njc4OTo7PD0+P0BBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWltcXV5fYGFiY2Rl
\t\tZmdoaWprbG1ub3BxcnN0dXZ3eHl6e3x9fn8=
\t\t</data>
\t</array>
\t<key>Death</key>
\t<integer>1564</integer>
\t<key>Height</key>
\t<real>1.6</real>
\t<key>Data</key>
\t<data>
\tAAAAvgAAAAMAAAAeAAAA
\t</data>
\t<key>Birthdate</key>
\t<date>1981-05-16T11:32:06Z</date>
\t<key>Comment</key>
\t<string>2 &lt; 3</string>
\t<key>BiggestNumber</key>
\t<integer>18446744073709551615</integer>
\t<key>SmallestNumber</key>
\t<integer>-9223372036854775808</integer>
\t<key>IsTrue</key>
\t<true/>
\t<key>IsNotFalse</key>
\t<false/>
</dict>
</plist>" ; let actual = events_to_xml (plist , XmlWriteOptions :: default ()) ; assert_eq ! (actual , expected) ; } # [test] fn custom_indent_string () { let plist = [Event :: StartArray (None) , Event :: String ("It is a tale told by an idiot," . into ()) , Event :: String ("Full of sound and fury, signifying nothing." . into ()) , Event :: EndCollection ,] ; let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<array>
...<string>It is a tale told by an idiot,</string>
...<string>Full of sound and fury, signifying nothing.</string>
</array>
</plist>" ; let actual = events_to_xml (plist , XmlWriteOptions :: default () . indent (b'.' , 3)) ; assert_eq ! (actual , expected) ; } # [test] fn no_root () { let plist = [Event :: StartArray (None) , Event :: String ("It is a tale told by an idiot," . into ()) , Event :: String ("Full of sound and fury, signifying nothing." . into ()) , Event :: EndCollection ,] ; let expected = "<array>
\t<string>It is a tale told by an idiot,</string>
\t<string>Full of sound and fury, signifying nothing.</string>
</array>" ; let actual = events_to_xml (plist , XmlWriteOptions :: default () . root_element (false)) ; assert_eq ! (actual , expected) ; } fn events_to_xml < 'event > (events : impl IntoIterator < Item = Event < 'event > > , options : XmlWriteOptions ,) -> String { let mut cursor = Cursor :: new (Vec :: new ()) ; let mut writer = XmlWriter :: new_with_options (& mut cursor , & options) ; for event in events { writer . write (event) . unwrap () ; } String :: from_utf8 (cursor . into_inner ()) . unwrap () } }
};
}
