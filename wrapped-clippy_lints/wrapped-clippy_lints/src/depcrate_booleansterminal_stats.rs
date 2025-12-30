// Generated macro for terminal_stats (function)
macro_rules! Depcrate_booleansterminal_stats {
() => {
// Module: crate::booleans
// Provides: {"terminal_stats"}
// Dependencies: {}
fn terminal_stats (b : & Bool) -> Stats { fn recurse (b : & Bool , stats : & mut Stats) { match b { True | False => stats . ops += 1 , Not (inner) => { match * * inner { And (_) | Or (_) => stats . ops += 1 , _ => stats . negations += 1 , } recurse (inner , stats) ; } , And (v) | Or (v) => { stats . ops += v . len () - 1 ; for inner in v { recurse (inner , stats) ; } } , & Term (n) => stats . terminals [n as usize] += 1 , } } use quine_mc_cluskey :: Bool :: { And , False , Not , Or , Term , True } ; let mut stats = Stats :: default () ; recurse (b , & mut stats) ; stats }
};
}
