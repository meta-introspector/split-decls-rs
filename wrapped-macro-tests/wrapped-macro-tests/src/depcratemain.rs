// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { use futures04 :: { executor :: block_on , future } ; block_on (async { let _ = futures04 :: join ! (async { } , async { }) ; let _ = macro_reexport :: join ! (async { } , async { }) ; let _ = macro_reexport :: join2 ! (async { } , async { }) ; }) ; let _ = block_on (async { let _ = futures04 :: try_join ! (async { Ok ::< () , () > (()) } , async { Ok ::< () , () > (()) }) ; let _ = macro_reexport :: try_join ! (async { Ok ::< () , () > (()) } , async { Ok ::< () , () > (()) }) ; let _ = macro_reexport :: try_join2 ! (async { Ok ::< () , () > (()) } , async { Ok ::< () , () > (()) }) ; Ok :: < () , () > (()) }) ; block_on (async { let mut a = future :: ready (()) ; let mut b = future :: pending :: < () > () ; futures04 :: select ! { _ = a => { } , _ = b => unreachable ! () , } ; let mut a = future :: ready (()) ; let mut b = future :: pending :: < () > () ; macro_reexport :: select ! { _ = a => { } , _ = b => unreachable ! () , } ; let mut a = future :: ready (()) ; let mut b = future :: pending :: < () > () ; macro_reexport :: select2 ! { _ = a => { } , _ = b => unreachable ! () , } ; }) ; block_on (async { let mut a = future :: ready (()) ; let mut b = future :: pending :: < () > () ; futures04 :: select_biased ! { _ = a => { } , _ = b => unreachable ! () , } ; let mut a = future :: ready (()) ; let mut b = future :: pending :: < () > () ; macro_reexport :: select_biased ! { _ = a => { } , _ = b => unreachable ! () , } ; let mut a = future :: ready (()) ; let mut b = future :: pending :: < () > () ; macro_reexport :: select_biased2 ! { _ = a => { } , _ = b => unreachable ! () , } ; }) ; }
};
}
