// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { let watcher = DeviceInformation :: CreateWatcher () ? ; watcher . Added (& TypedEventHandler :: < DeviceWatcher , DeviceInformation > :: new (| _ , info | { println ! ("{:?}" , info . as_ref () . expect ("info") . Name () ?) ; Ok (()) } ,)) ? ; watcher . EnumerationCompleted (& TypedEventHandler :: new (| _ , _ | { println ! ("done!") ; Ok (()) })) ? ; watcher . Start () ? ; std :: thread :: sleep (std :: time :: Duration :: new (10 , 0)) ; Ok (()) }
};
}
