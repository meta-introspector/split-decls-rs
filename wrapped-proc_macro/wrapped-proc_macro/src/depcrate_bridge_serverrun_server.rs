// Generated macro for run_server (function)
macro_rules! Depcrate_bridge_serverrun_server {
() => {
// Module: crate::bridge::server
// Provides: {"run_server"}
// Dependencies: {}
fn run_server < S : Server , I : Encode < HandleStore < MarkedTypes < S > > > , O : for < 'a , 's > DecodeMut < 'a , 's , HandleStore < MarkedTypes < S > > > , > (strategy : & impl ExecutionStrategy , handle_counters : & 'static client :: HandleCounters , server : S , input : I , run_client : extern "C" fn (BridgeConfig < '_ >) -> Buffer , force_show_panics : bool ,) -> Result < O , PanicMessage > { let mut dispatcher = Dispatcher { handle_store : HandleStore :: new (handle_counters) , server : MarkedTypes (server) } ; let globals = dispatcher . server . globals () ; let mut buf = Buffer :: new () ; (globals , input) . encode (& mut buf , & mut dispatcher . handle_store) ; buf = strategy . run_bridge_and_client (& mut dispatcher , buf , run_client , force_show_panics) ; Result :: decode (& mut & buf [..] , & mut dispatcher . handle_store) }
};
}
