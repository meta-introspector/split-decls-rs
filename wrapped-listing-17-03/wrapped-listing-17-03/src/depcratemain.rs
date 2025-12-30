// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
async fn main () { let args : Vec < String > = std :: env :: args () . collect () ; let url = & args [1] ; match page_title (url) . await { Some (title) => println ! ("The title for {url} was {title}") , None => println ! ("{url} had no title") , } }
};
}
