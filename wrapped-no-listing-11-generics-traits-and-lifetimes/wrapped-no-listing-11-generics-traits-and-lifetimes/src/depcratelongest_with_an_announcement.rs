// Generated macro for longest_with_an_announcement (function)
macro_rules! Depcratelongest_with_an_announcement {
() => {
// Module: crate
// Provides: {"longest_with_an_announcement"}
// Dependencies: {}
fn longest_with_an_announcement < 'a , T > (x : & 'a str , y : & 'a str , ann : T ,) -> & 'a str where T : Display , { println ! ("Announcement! {ann}") ; if x . len () > y . len () { x } else { y } }
};
}
