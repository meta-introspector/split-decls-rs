// Generated macro for drive_uni (function)
macro_rules! Depcrate_clientdrive_uni {
() => {
// Module: crate::client
// Provides: {"drive_uni"}
// Dependencies: {}
async fn drive_uni (connection : quinn :: Connection , stream_stats : OpenStreamStats , concurrency : u64 , upload : u64 , download : u64 ,) -> Result < () > { if concurrency == 0 { return Ok (()) ; } let sem = Arc :: new (Semaphore :: new (concurrency as usize)) ; loop { let permit = sem . clone () . acquire_owned () . await . unwrap () ; let send = connection . open_uni () . await ? ; let stream_stats = stream_stats . clone () ; debug ! ("sending request on {}" , send . id ()) ; let connection = connection . clone () ; tokio :: spawn (async move { if let Err (e) = request_uni (send , connection , upload , download , stream_stats) . await { error ! ("sending request failed: {:#}" , e) ; } drop (permit) ; }) ; } }
};
}
