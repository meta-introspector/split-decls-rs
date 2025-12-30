// Generated macro for setup_tracing (function)
macro_rules! Depcrate_consteval_testssetup_tracing {
() => {
// Module: crate::consteval::tests
// Provides: {"setup_tracing"}
// Dependencies: {}
fn setup_tracing () -> Option < tracing :: subscriber :: DefaultGuard > { static ENABLE : LazyLock < bool > = LazyLock :: new (| | env :: var ("CHALK_DEBUG") . is_ok ()) ; if ! * ENABLE { return None ; } let filter : tracing_subscriber :: filter :: Targets = env :: var ("CHALK_DEBUG") . ok () . and_then (| it | it . parse () . ok ()) . unwrap_or_default () ; let layer = HierarchicalLayer :: default () . with_indent_lines (true) . with_ansi (false) . with_indent_amount (2) . with_writer (std :: io :: stderr) ; let subscriber = Registry :: default () . with (filter) . with (layer) ; Some (tracing :: subscriber :: set_default (subscriber)) }
};
}
