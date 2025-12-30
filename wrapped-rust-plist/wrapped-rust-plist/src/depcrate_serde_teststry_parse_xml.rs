// Generated macro for try_parse_xml (function)
macro_rules! Depcrate_serde_teststry_parse_xml {
() => {
// Module: crate::serde_tests
// Provides: {"try_parse_xml"}
// Dependencies: {}
fn try_parse_xml (bom : bool , whitespace : bool , decl : bool , comment : bool , doctype : bool) -> bool { # [derive (Deserialize)] struct LayerinfoData { color : Option < String > , } let mut data = Vec :: new () ; if bom { data . extend (b"\xef\xbb\xbf") ; } if whitespace { data . extend (b"\r\n\t ") ; } if decl { data . extend (br#"<?xml version="1.0" encoding="UTF-8"?>"#) ; } if comment { data . extend (br#"<!-- hello -->"#) ; } if doctype { data . extend (br#"<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">"#) ; } data . extend (br#"<plist version="1.0">
        <dict>
            <key>color</key>
            <string>1,0.75,0,0.7</string>
        </dict>
        </plist>
        "# ,) ; if let Ok (lib_dict) = crate :: from_bytes :: < LayerinfoData > (& data) { lib_dict . color . unwrap () == "1,0.75,0,0.7" } else { false } }
};
}
