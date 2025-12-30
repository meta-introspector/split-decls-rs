// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Box < dyn Error + Send + Sync > > { let opt = Opt :: parse () ; let profiling_data = ProfilingData :: new (& opt . file_prefix) ? ; let recorded_stacks = collapse_stacks (& profiling_data) . iter () . map (| (unique_stack , count) | format ! ("{} {}" , unique_stack , count)) . collect :: < Vec < _ > > () ; let file = BufWriter :: new (File :: create ("rustc.svg") ?) ; let mut flamegraph_options = FlamegraphOptions :: default () ; from_lines (& mut flamegraph_options , recorded_stacks . iter () . map (| s | s . as_ref ()) , file ,) . expect ("unable to generate a flamegraph \
         from the collapsed stack data" ,) ; Ok (()) }
};
}
