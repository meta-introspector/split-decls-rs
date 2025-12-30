// Generated macro for tests (module)
macro_rules! Depcrate_greentests {
() => {
// Module: crate::green
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn assert_send_sync () { fn f < T : Send + Sync > () { } f :: < GreenNode > () ; f :: < GreenToken > () ; f :: < GreenElement > () ; } # [test] fn test_size_of () { use std :: mem :: size_of ; eprintln ! ("GreenNode          {}" , size_of ::< GreenNode > ()) ; eprintln ! ("GreenToken         {}" , size_of ::< GreenToken > ()) ; eprintln ! ("GreenElement       {}" , size_of ::< GreenElement > ()) ; } }
};
}
