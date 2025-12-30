// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let args : Vec < String > = std :: env :: args () . collect () ; trpl :: block_on (async { let title_fut_1 = page_title (& args [1]) ; let title_fut_2 = page_title (& args [2]) ; let (url , maybe_title) = match trpl :: select (title_fut_1 , title_fut_2) . await { Either :: Left (left) => left , Either :: Right (right) => right , } ; println ! ("{url} returned first") ; match maybe_title { Some (title) => println ! ("Its page title was: '{title}'") , None => println ! ("It had no title.") , } }) }
};
}
