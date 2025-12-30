// Generated macro for Parser (struct)
macro_rules! Depcrate_content_yaml_vendored_parserParser {
() => {
// Module: crate::content::yaml::vendored::parser
// Provides: {"Parser"}
// Dependencies: {}
# [derive (Debug)] pub struct Parser < T > { scanner : Scanner < T > , states : Vec < State > , state : State , token : Option < Token > , current : Option < (Event , Marker) > , anchors : HashMap < String , usize > , anchor_id : usize , }
};
}
