// Generated macro for Command (enum)
macro_rules! DepcrateCommand {
() => {
// Module: crate
// Provides: {"Command"}
// Dependencies: {}
# [derive (Subcommand)] pub enum Command { # [doc = " Run all benchmarks and print the measured CPU instruction counts in CSV format"] RunAll { # [arg (short , long , default_value = "target/ci-bench")] output_dir : PathBuf , } , # [doc = " Run a single benchmark at the provided index (used by the bench runner to start each benchmark in its own process)"] RunSingle { index : u32 , side : Side , measurement_mode : Mode , } , # [doc = " Run all benchmarks in walltime mode and print the measured timings in CSV format"] Walltime { # [arg (short , long)] iterations_per_scenario : usize , } , # [doc = " Compare the icount results from two previous benchmark runs and print a user-friendly markdown overview"] Compare { # [doc = " Path to the directory with the results of a previous `run-all` execution"] baseline_dir : PathBuf , # [doc = " Path to the directory with the results of a previous `run-all` execution"] candidate_dir : PathBuf , } , # [doc = " Compare the memory results from two previous benchmark runs and print a user-friendly markdown overview"] CompareMemory { comparator : CompareMemoryOperand , # [doc = " Path to the directory with the results of a previous `run-all` execution"] baseline_dir : PathBuf , # [doc = " Path to the directory with the results of a previous `run-all` execution"] candidate_dir : PathBuf , } , }
};
}
