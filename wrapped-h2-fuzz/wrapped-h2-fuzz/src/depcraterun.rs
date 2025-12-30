// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
async fn run (script : & [u8]) -> Result < () , h2 :: Error > { let io = MockIo { input : script } ; let (mut h2 , mut connection) = h2 :: client :: handshake (io) . await ? ; let mut futs = FuturesUnordered :: new () ; let future = future :: poll_fn (| cx | { if let Poll :: Ready (()) = Pin :: new (& mut connection) . poll (cx) ? { return Poll :: Ready (Ok :: < _ , h2 :: Error > (())) ; } while futs . len () < 128 { if ! h2 . poll_ready (cx) ? . is_ready () { break ; } let request = Request :: builder () . method (Method :: POST) . uri ("https://example.com/") . body (()) . unwrap () ; let (resp , mut send) = h2 . send_request (request , false) ? ; send . send_data (vec ! [0u8 ; 32769] . into () , true) . unwrap () ; drop (send) ; futs . push (resp) ; } loop { match Pin :: new (& mut futs) . poll_next (cx) { Poll :: Pending | Poll :: Ready (None) => break , r @ Poll :: Ready (Some (Ok (_))) | r @ Poll :: Ready (Some (Err (_))) => { eprintln ! ("{:?}" , r) ; } } } Poll :: Pending }) ; future . await ? ; Ok (()) }
};
}
