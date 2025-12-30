// Generated macro for do_paste_name_value_attr (function)
macro_rules! Depcrate_attrdo_paste_name_value_attr {
() => {
// Module: crate::attr
// Provides: {"do_paste_name_value_attr"}
// Dependencies: {}
fn do_paste_name_value_attr (attr : TokenStream , span : Span , leading : usize) -> Result < TokenStream > { let mut expanded = TokenStream :: new () ; let mut tokens = attr . into_iter () . peekable () ; expanded . extend (tokens . by_ref () . take (leading + 1)) ; let mut segments = segment :: parse (& mut tokens) ? ; for segment in & mut segments { if let Segment :: String (string) = segment { if let Some (open_quote) = string . value . find ('"') { if open_quote == 0 { string . value . truncate (string . value . len () - 1) ; string . value . remove (0) ; } else { let begin = open_quote + 1 ; let end = string . value . rfind ('"') . unwrap () ; let raw_string = mem :: replace (& mut string . value , String :: new ()) ; for ch in raw_string [begin .. end] . chars () { string . value . extend (ch . escape_default ()) ; } } } } } let mut lit = segment :: paste (& segments) ? ; lit . insert (0 , '"') ; lit . push ('"') ; let mut lit = TokenStream :: from_str (& lit) . unwrap () . into_iter () . next () . unwrap () ; lit . set_span (span) ; expanded . extend (iter :: once (lit)) ; Ok (expanded) }
};
}
