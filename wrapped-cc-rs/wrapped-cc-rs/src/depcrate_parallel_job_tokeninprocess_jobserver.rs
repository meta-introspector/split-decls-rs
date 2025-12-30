// Generated macro for inprocess_jobserver (module)
macro_rules! Depcrate_parallel_job_tokeninprocess_jobserver {
() => {
// Module: crate::parallel::job_token
// Provides: {"inprocess_jobserver"}
// Dependencies: {}
mod inprocess_jobserver { use super :: JobToken ; use crate :: parallel :: async_executor :: YieldOnce ; use std :: { env :: var , sync :: atomic :: { AtomicU32 , Ordering :: { AcqRel , Acquire } , } , } ; pub (crate) struct JobServer (AtomicU32) ; impl JobServer { # [allow (clippy :: disallowed_methods)] pub (super) fn new () -> Self { let parallelism = var ("NUM_JOBS") . ok () . and_then (| j | j . parse :: < u32 > () . ok ()) . or_else (| | Some (std :: thread :: available_parallelism () . ok () ? . get () as u32)) . unwrap_or (4) ; Self (AtomicU32 :: new (parallelism)) } pub (super) async fn acquire (& self) -> JobToken { loop { let res = self . 0 . fetch_update (AcqRel , Acquire , | tokens | tokens . checked_sub (1)) ; if res . is_ok () { break JobToken :: new () ; } YieldOnce :: default () . await } } pub (super) fn release_token_raw (& self) { self . 0 . fetch_add (1 , AcqRel) ; } } }
};
}
