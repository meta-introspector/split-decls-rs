// Generated macro for run_client (function)
macro_rules! Depcrate_bridge_clientrun_client {
() => {
// Module: crate::bridge::client
// Provides: {"run_client"}
// Dependencies: {}
# [doc = " Client-side helper for handling client panics, entering the bridge,"] # [doc = " deserializing input and serializing output."] fn run_client < A : for < 'a , 's > DecodeMut < 'a , 's , () > , R : Encode < () > > (config : BridgeConfig < '_ > , f : impl FnOnce (A) -> R ,) -> Buffer { let BridgeConfig { input : mut buf , dispatch , force_show_panics , .. } = config ; panic :: catch_unwind (panic :: AssertUnwindSafe (| | { maybe_install_panic_hook (force_show_panics) ; Symbol :: invalidate_all () ; let reader = & mut & buf [..] ; let (globals , input) = < (ExpnGlobals < Span > , A) > :: decode (reader , & mut ()) ; let state = RefCell :: new (Bridge { cached_buffer : buf . take () , dispatch , globals }) ; let output = state :: set (& state , | | f (input)) ; buf = RefCell :: into_inner (state) . cached_buffer ; buf . clear () ; Ok :: < _ , () > (output) . encode (& mut buf , & mut ()) ; })) . map_err (PanicMessage :: from) . unwrap_or_else (| e | { buf . clear () ; Err :: < () , _ > (e) . encode (& mut buf , & mut ()) ; }) ; Symbol :: invalidate_all () ; buf }
};
}
