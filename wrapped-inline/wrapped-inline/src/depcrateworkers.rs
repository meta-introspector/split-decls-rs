// Generated macro for workers (function)
macro_rules! Depcrateworkers {
() => {
// Module: crate
// Provides: {"workers"}
// Dependencies: {}
# [expect (clippy :: cast_precision_loss , clippy :: needless_pass_by_value)] fn workers (tx : mpsc :: Sender < Event >) -> Vec < Worker > { (0 .. 4) . map (| id | { let (worker_tx , worker_rx) = mpsc :: channel :: < Download > () ; let tx = tx . clone () ; thread :: spawn (move | | { while let Ok (download) = worker_rx . recv () { let mut remaining = download . size ; while remaining > 0 { let wait = (remaining as u64) . min (10) ; thread :: sleep (Duration :: from_millis (wait * 10)) ; remaining = remaining . saturating_sub (10) ; let progress = (download . size - remaining) * 100 / download . size ; tx . send (Event :: DownloadUpdate (id , download . id , progress as f64)) . unwrap () ; } tx . send (Event :: DownloadDone (id , download . id)) . unwrap () ; } }) ; Worker { id , tx : worker_tx } }) . collect () }
};
}
