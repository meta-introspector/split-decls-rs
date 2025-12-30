// Generated macro for MimeGuess (struct)
macro_rules! DepcrateMimeGuess {
() => {
// Module: crate
// Provides: {"MimeGuess"}
// Dependencies: {}
# [doc = " A \"guess\" of the MIME/Media Type(s) of an extension or path as one or more"] # [doc = " [`Mime`](struct.Mime.html) instances."] # [doc = ""] # [doc = " ### Note: Ordering"] # [doc = " A given file format may have one or more applicable Media Types; in this case"] # [doc = " the first Media Type returned is whatever is declared in the latest IETF RFC for the"] # [doc = " presumed file format or the one that explicitly supercedes all others."] # [doc = " Ordering of additional Media Types is arbitrary."] # [doc = ""] # [doc = " ### Note: Values Not Stable"] # [doc = " The exact Media Types returned in any given guess are not considered to be stable and are often"] # [doc = " updated in patch releases in order to reflect the most up-to-date information possible."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct MimeGuess (& 'static [& 'static str]) ;
};
}
