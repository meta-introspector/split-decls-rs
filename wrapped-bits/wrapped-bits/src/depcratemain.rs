// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { unsafe { CoInitializeEx (None , COINIT_MULTITHREADED) . ok () ? ; let manager : IBackgroundCopyManager = CoCreateInstance (& BackgroundCopyManager , None , CLSCTX_LOCAL_SERVER) ? ; let mut job = None ; manager . CreateJob (w ! ("sample") , BG_JOB_TYPE_DOWNLOAD , & mut Default :: default () , & mut job ,) ? ; let job = job . unwrap () ; job . AddFile (w ! ("https://kennykerr.ca/favicon.svg") , w ! ("D:\\rust.svg")) ? ; let callback : IBackgroundCopyCallback = Callback . into () ; job . SetNotifyInterface (& callback) ? ; job . SetNotifyFlags (BG_NOTIFY_JOB_TRANSFERRED | BG_NOTIFY_JOB_ERROR) ? ; job . Resume () ? ; println ! ("downloading...") ; getchar () ; job . Cancel () ? ; println ! ("canceled") ; Ok (()) } }
};
}
