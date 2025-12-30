// Generated macro for should_implement_is_future_await_logic (module)
macro_rules! Depcrate_parse_argumentsshould_implement_is_future_await_logic {
() => {
// Module: crate::parse::arguments
// Provides: {"should_implement_is_future_await_logic"}
// Dependencies: {}
# [cfg (test)] mod should_implement_is_future_await_logic { use super :: * ; use crate :: test :: * ; # [fixture] fn info () -> ArgumentsInfo { let mut a = ArgumentsInfo :: default () ; a . set_future (pat ("simple") , FutureArg :: Define) ; a . set_future (pat ("other_simple") , FutureArg :: Define) ; a . set_future (pat ("awaited") , FutureArg :: Await) ; a . set_future (pat ("other_awaited") , FutureArg :: Await) ; a . set_future (pat ("none") , FutureArg :: None) ; a } # [rstest] fn no_matching_ident (info : ArgumentsInfo) { assert ! (! info . is_future_await (& pat ("some"))) ; assert ! (! info . is_future_await (& pat ("simple"))) ; assert ! (! info . is_future_await (& pat ("none"))) ; } # [rstest] fn matching_ident (info : ArgumentsInfo) { assert ! (info . is_future_await (& pat ("awaited"))) ; assert ! (info . is_future_await (& pat ("other_awaited"))) ; } # [rstest] fn global_matching_future_ident (mut info : ArgumentsInfo) { info . set_global_await (true) ; assert ! (info . is_future_await (& pat ("simple"))) ; assert ! (info . is_future_await (& pat ("other_simple"))) ; assert ! (info . is_future_await (& pat ("awaited"))) ; assert ! (! info . is_future_await (& pat ("some"))) ; assert ! (! info . is_future_await (& pat ("none"))) ; } }
};
}
