// Generated macro for aws_lc_rs_ticketer (function)
macro_rules! Depcrateaws_lc_rs_ticketer {
() => {
// Module: crate
// Provides: {"aws_lc_rs_ticketer"}
// Dependencies: {}
fn aws_lc_rs_ticketer () -> Arc < dyn TicketProducer > { aws_lc_rs :: DEFAULT_PROVIDER . ticketer_factory . ticketer () . unwrap () }
};
}
