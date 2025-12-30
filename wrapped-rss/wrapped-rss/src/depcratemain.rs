// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { let uri = Uri :: CreateUri (h ! ("https://blogs.windows.com/feed")) ? ; let client = SyndicationClient :: new () ? ; client . SetRequestHeader (h ! ("User-Agent") , h ! ("Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; WOW64; Trident/6.0)") ,) ? ; let feed = client . RetrieveFeedAsync (& uri) ? . join () ? ; for item in feed . Items () ? { println ! ("{:?}" , item . Title () ?. Text () ?) ; } Ok (()) }
};
}
