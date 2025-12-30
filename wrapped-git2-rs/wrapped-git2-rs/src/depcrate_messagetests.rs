// Generated macro for tests (module)
macro_rules! Depcrate_messagetests {
() => {
// Module: crate::message
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn prettify () { use crate :: { message_prettify , DEFAULT_COMMENT_CHAR } ; assert_eq ! (message_prettify ("1\n\n\n2" , None) . unwrap () , "1\n\n2\n") ; assert_eq ! (message_prettify ("1\n\n\n2\n\n\n3" , None) . unwrap () , "1\n\n2\n\n3\n") ; assert_eq ! (message_prettify ("1\n# comment\n# more" , None) . unwrap () , "1\n# comment\n# more\n") ; assert_eq ! (message_prettify ("1\n# comment\n# more" , DEFAULT_COMMENT_CHAR) . unwrap () , "1\n") ; assert_eq ! (message_prettify ("1\n; comment\n; more" , Some (';' as u8)) . unwrap () , "1\n") ; } # [test] fn trailers () { use crate :: { message_trailers_bytes , message_trailers_strs , MessageTrailersStrs } ; use std :: collections :: HashMap ; let message1 = "
WHAT ARE WE HERE FOR

What are we here for?

Just to be eaten?
" ; let expected : HashMap < & str , & str > = HashMap :: new () ; assert_eq ! (expected , to_map (& message_trailers_strs (message1) . unwrap ())) ; let message2 = "
Attention all

We are out of tomatoes.

Spoken-by: Major Turnips
Transcribed-by: Seargant Persimmons
Signed-off-by: Colonel Kale
" ; let expected : HashMap < & str , & str > = vec ! [("Spoken-by" , "Major Turnips") , ("Transcribed-by" , "Seargant Persimmons") , ("Signed-off-by" , "Colonel Kale") ,] . into_iter () . collect () ; assert_eq ! (expected , to_map (& message_trailers_strs (message2) . unwrap ())) ; let message3 = "
The fate of Seargant Green-Peppers

Seargant Green-Peppers was killed by Caterpillar Battalion 44.

Signed-off-by: Colonel Kale
---
I never liked that guy, anyway.

Opined-by: Corporal Garlic
" ; let expected : HashMap < & str , & str > = vec ! [("Signed-off-by" , "Colonel Kale")] . into_iter () . collect () ; assert_eq ! (expected , to_map (& message_trailers_strs (message3) . unwrap ())) ; let message4 = b"
Be honest guys

Am I a malformed brussels sprout?

Signed-off-by: Lieutenant \xe2\x28\xa1prout
" ; let trailer = message_trailers_bytes (& message4 [..]) . unwrap () ; let expected = (& b"Signed-off-by" [..] , & b"Lieutenant \xe2\x28\xa1prout" [..]) ; let actual = trailer . iter () . next () . unwrap () ; assert_eq ! (expected , actual) ; fn to_map (trailers : & MessageTrailersStrs) -> HashMap < & str , & str > { let mut map = HashMap :: with_capacity (trailers . len ()) ; for (key , value) in trailers . iter () { map . insert (key , value) ; } map } } }
};
}
