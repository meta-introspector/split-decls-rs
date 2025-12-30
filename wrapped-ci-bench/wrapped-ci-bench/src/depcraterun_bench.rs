// Generated macro for run_bench (function)
macro_rules! Depcraterun_bench {
() => {
// Module: crate
// Provides: {"run_bench"}
// Dependencies: {}
# [doc = " Runs the benchmark using the provided stepper"] async fn run_bench < T : BenchStepper > (mut stepper : T , kind : BenchmarkKind , resumed_reps : usize ,) -> anyhow :: Result < () > { match kind { BenchmarkKind :: Handshake (ResumptionKind :: No) => { let _count = CountInstructions :: start () ; black_box (stepper . handshake () . await ?) ; } BenchmarkKind :: Handshake (_) => { stepper . handshake () . await ? ; let _count = CountInstructions :: start () ; for _ in 0 .. resumed_reps { stepper . sync_before_resumed_handshake () . await ? ; let endpoint = stepper . handshake () . await ? ; assert_eq ! (stepper . handshake_kind (& endpoint) , HandshakeKind :: Resumed) ; } } BenchmarkKind :: Transfer => { let mut endpoint = stepper . handshake () . await ? ; let _count = CountInstructions :: start () ; stepper . transmit_data (& mut endpoint) . await ? ; } } Ok (()) }
};
}
