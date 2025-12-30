// Generated macro for drive_bi (function)
macro_rules! Depcrate_clientdrive_bi {
() => {
// Module: crate::client
// Provides: {"drive_bi"}
// Dependencies: {}
async fn drive_bi (connection : quinn :: Connection , stream_stats : OpenStreamStats , concurrency : u64 , upload : u64 , download : u64 ,) -> Result < () > { if concurrency == 0 { return Ok (()) ; } let sem = Arc :: new (Semaphore :: new (concurrency as usize)) ; loop { let permit = sem . clone () . acquire_owned () . await . unwrap () ; let (send , recv) = connection . open_bi () . await ? ; let stream_stats = stream_stats . clone () ; debug ! ("sending request on {}" , send . id ()) ; tokio :: spawn (async move { if let Err (e) = request_bi (send , recv , upload , download , stream_stats) . await { error ! ("request failed: {:#}" , e) ; } drop (permit) ; }) ; } }
};
}
