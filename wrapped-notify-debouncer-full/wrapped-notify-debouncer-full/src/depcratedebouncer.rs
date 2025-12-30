// Generated macro for Debouncer (struct)
macro_rules! DepcrateDebouncer {
() => {
// Module: crate
// Provides: {"Debouncer"}
// Dependencies: {}
# [doc = " Debouncer guard, stops the debouncer on drop."] # [derive (Debug)] pub struct Debouncer < T : Watcher , C : FileIdCache > { watcher : T , debouncer_thread : Option < std :: thread :: JoinHandle < () > > , data : DebounceData < C > , stop : Arc < AtomicBool > , }
};
}
