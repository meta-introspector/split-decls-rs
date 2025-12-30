// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
pub fn run () -> Box < dyn Future < Item = () , Error = Error > > { let c2s = Dummy ([0u8 ; BUFFER_SIZE]) . then (move | _ | Ok (0)) ; let s2c = Dummy (()) . then (move | _ | Ok (0)) ; let fut = c2s . select (s2c) . and_then (move | _ | Ok (())) . map_err (| (err , _) | err) ; Box :: new (fut) }
};
}
