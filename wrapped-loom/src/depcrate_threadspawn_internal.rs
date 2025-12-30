// Generated macro for spawn_internal (function)
macro_rules! Depcrate_threadspawn_internal {
() => {
// Module: crate::thread
// Provides: {"spawn_internal"}
// Dependencies: {}
fn spawn_internal < F , T > (f : F , name : Option < String > , stack_size : Option < usize > , location : Location ,) -> JoinHandle < T > where F : FnOnce () -> T , F : 'static , T : 'static , { let result = Arc :: new (Mutex :: new (None)) ; let notify = rt :: Notify :: new (true , false) ; let id = { let name = name . clone () ; let result = result . clone () ; rt :: spawn (stack_size , move | | { rt :: execution (| execution | { init_current (execution , name) ; }) ; * result . lock () . unwrap () = Some (Ok (f ())) ; notify . notify (location) ; }) } ; JoinHandle { result , notify , thread : Thread { id : ThreadId { id } , name , } , } }
};
}
