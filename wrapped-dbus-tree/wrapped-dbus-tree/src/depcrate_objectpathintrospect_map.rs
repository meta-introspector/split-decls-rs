// Generated macro for introspect_map (function)
macro_rules! Depcrate_objectpathintrospect_map {
() => {
// Module: crate::objectpath
// Provides: {"introspect_map"}
// Dependencies: {}
fn introspect_map < I : fmt :: Display , T : Introspect > (h : & ArcMap < I , T > , indent : & str) -> String { h . iter () . fold ("" . into () , | a , (k , v) | { let (name , params , contents) = (v . xml_name () , v . xml_params () , v . xml_contents ()) ; format ! ("{}{}<{} name=\"{}\"{}{}>\n" , a , indent , name , &* k , params , if ! contents . is_empty () { format ! (">\n{}{}</{}" , contents , indent , name) } else { "/" . to_string () }) }) }
};
}
