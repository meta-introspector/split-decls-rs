// Generated macro for DefaultHandler (struct)
macro_rules! DepcrateDefaultHandler {
() => {
// Module: crate
// Provides: {"DefaultHandler"}
// Dependencies: {}
# [doc = " The default provided error report handler for `eyre::Report`."] # [doc = ""] # [doc = " On nightly this supports conditionally capturing a `std::backtrace::Backtrace` if the source"] # [doc = " error did not already capture one."] # [allow (dead_code)] pub struct DefaultHandler { backtrace : Option < Backtrace > , # [cfg (track_caller)] location : Option < & 'static std :: panic :: Location < 'static > > , }
};
}
