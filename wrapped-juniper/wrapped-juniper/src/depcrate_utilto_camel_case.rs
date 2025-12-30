// Generated macro for to_camel_case (function)
macro_rules! Depcrate_utilto_camel_case {
() => {
// Module: crate::util
// Provides: {"to_camel_case"}
// Dependencies: {}
# [doc = " Convert string to camel case."] # [doc = ""] # [doc = " Note: needs to be public because several macros use it."] # [doc (hidden)] pub fn to_camel_case (s : & '_ str) -> Cow < '_ , str > { let mut dest = Cow :: Borrowed (s) ; let s_iter = if let Some (stripped) = s . strip_prefix ('_') { stripped } else { s } . split ('_') . enumerate () ; for (i , part) in s_iter { if i > 0 && part . len () == 1 { dest += Cow :: Owned (part . to_uppercase ()) ; } else if i > 0 && part . len () > 1 { let first = part . chars () . next () . unwrap () . to_uppercase () . collect :: < String > () ; let second = & part [1 ..] ; dest += Cow :: Owned (first) ; dest += second ; } else if i == 0 { dest = Cow :: Borrowed (part) ; } } dest }
};
}
