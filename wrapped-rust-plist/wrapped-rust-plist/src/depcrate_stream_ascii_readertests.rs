// Generated macro for tests (module)
macro_rules! Depcrate_stream_ascii_readertests {
() => {
// Module: crate::stream::ascii_reader
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: fs :: File ; use super :: * ; use crate :: stream :: Event :: * ; # [test] fn empty_test () { let plist = "" ; let streaming_parser = AsciiReader :: new (plist . as_bytes ()) ; let events : Vec < Event > = streaming_parser . map (| e | e . unwrap ()) . collect () ; assert_eq ! (events , & []) ; } # [test] fn streaming_sample () { let reader = File :: open ("./tests/data/ascii-sample.plist") . unwrap () ; let streaming_parser = AsciiReader :: new (reader) ; let events : Vec < Event > = streaming_parser . map (| e | e . unwrap ()) . collect () ; let comparison = & [StartDictionary (None) , String ("KeyName1" . into ()) , String ("Value1" . into ()) , String ("AnotherKeyName" . into ()) , String ("Value2" . into ()) , String ("Something" . into ()) , StartArray (None) , String ("ArrayItem1" . into ()) , String ("ArrayItem2" . into ()) , String ("ArrayItem3" . into ()) , EndCollection , String ("Key4" . into ()) , String ("0.10" . into ()) , String ("KeyFive" . into ()) , StartDictionary (None) , String ("Dictionary2Key1" . into ()) , String ("Something" . into ()) , String ("AnotherKey" . into ()) , String ("Somethingelse" . into ()) , EndCollection , EndCollection ,] ; assert_eq ! (events , comparison) ; } # [test] fn utf8_strings () { let plist = "{ names = (Léa, François, Żaklina, 王芳); }" ; let streaming_parser = AsciiReader :: new (plist . as_bytes ()) ; let events : Vec < Event > = streaming_parser . map (| e | e . unwrap ()) . collect () ; let comparison = & [StartDictionary (None) , String ("names" . into ()) , StartArray (None) , String ("Léa" . into ()) , String ("François" . into ()) , String ("Żaklina" . into ()) , String ("王芳" . into ()) , EndCollection , EndCollection ,] ; assert_eq ! (events , comparison) ; } # [test] fn invalid_utf16_escapes () { let plist = br#"{
            key1 = "\U123";
            key2 = "\UD83D";
            key3 = "\u0080";
        }"# ; let streaming_parser = AsciiReader :: new (& plist [..]) ; let events : Vec < Result < Event , Error > > = streaming_parser . collect () ; assert ! (events [2] . is_err ()) ; assert ! (events [4] . is_err ()) ; assert ! (events [6] . is_err ()) ; } # [test] fn invalid_octal_escapes () { let plist = br#"{
            key1 = "\1";
            key2 = "\12";
        }"# ; let streaming_parser = AsciiReader :: new (& plist [..]) ; let events : Vec < Result < Event , Error > > = streaming_parser . collect () ; assert ! (events [2] . is_err ()) ; assert ! (events [4] . is_err ()) ; } # [test] fn escaped_sequences_in_strings () { let plist = br#"{
            key1 = "va\"lue";
            key2 = 'va"lue';
            key3 = "va\a\b\f\n\r\t\v\"\nlue";
            key4 = "a\012b";
            key5 = "\\UD83D\\UDCA9";
            key6 = "\UD83D\UDCA9";
            key7 = "\U0080";
            key8 = "\200\377";
        }"# ; let streaming_parser = AsciiReader :: new (& plist [..]) ; let events : Vec < Event > = streaming_parser . map (| e | e . unwrap ()) . collect () ; let comparison = & [StartDictionary (None) , String ("key1" . into ()) , String (r#"va"lue"# . into ()) , String ("key2" . into ()) , String (r#"va"lue"# . into ()) , String ("key3" . into ()) , String ("va\u{7}\u{8}\u{c}\n\r\t\u{b}\"\nlue" . into ()) , String ("key4" . into ()) , String ("a\nb" . into ()) , String ("key5" . into ()) , String ("\\UD83D\\UDCA9" . into ()) , String ("key6" . into ()) , String ("💩" . into ()) , String ("key7" . into ()) , String ("\u{80}" . into ()) , String ("key8" . into ()) , String ("\u{a0}\u{fffd}" . into ()) , EndCollection ,] ; assert_eq ! (events , comparison) ; } # [test] fn integers_and_strings () { let plist = b"{ name = James, age = 42 }" ; let streaming_parser = AsciiReader :: new (& plist [..]) ; let events : Vec < Event > = streaming_parser . map (| e | e . unwrap ()) . collect () ; let comparison = & [StartDictionary (None) , String ("name" . into ()) , String ("James" . into ()) , String ("age" . into ()) , Integer (42 . into ()) , EndCollection ,] ; assert_eq ! (events , comparison) ; } # [test] fn netnewswire_pbxproj () { let reader = File :: open ("./tests/data/netnewswire.pbxproj") . unwrap () ; let streaming_parser = AsciiReader :: new (reader) ; let events : Vec < Event > = streaming_parser . map (| e | e . unwrap ()) . collect () ; assert ! (! events . is_empty ()) ; } # [test] fn multiple_unquoted_strings () { let plist = b"not a plist" ; let streaming_parser = AsciiReader :: new (& plist [..]) ; let events : Vec < Event > = streaming_parser . map (| e | e . unwrap ()) . collect () ; let comparison = & [String ("not" . into ()) , String ("a" . into ()) , String ("plist" . into ()) ,] ; assert_eq ! (events , comparison) ; } }
};
}
