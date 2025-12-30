// Generated macro for dictionary_deserialize_dictionary_in_struct (function)
macro_rules! Depcrate_serde_testsdictionary_deserialize_dictionary_in_struct {
() => {
// Module: crate::serde_tests
// Provides: {"dictionary_deserialize_dictionary_in_struct"}
// Dependencies: {}
# [test] fn dictionary_deserialize_dictionary_in_struct () { # [derive (Deserialize)] struct LayerinfoData { color : Option < String > , lib : Option < Dictionary > , } let lib_dict : LayerinfoData = crate :: from_bytes (r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>color</key>
            <string>1,0.75,0,0.7</string>
            <key>lib</key>
            <dict>
            <key>com.typemytype.robofont.segmentType</key>
            <string>curve</string>
            </dict>
        </dict>
        </plist>
        "# . as_bytes ()) . unwrap () ; assert_eq ! (lib_dict . color . unwrap () , "1,0.75,0,0.7") ; assert_eq ! (lib_dict . lib . unwrap () . get ("com.typemytype.robofont.segmentType") . unwrap () . as_string () . unwrap () , "curve") ; }
};
}
