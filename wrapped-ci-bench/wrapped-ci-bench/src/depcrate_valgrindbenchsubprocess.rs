// Generated macro for BenchSubprocess (struct)
macro_rules! Depcrate_valgrindBenchSubprocess {
() => {
// Module: crate::valgrind
// Provides: {"BenchSubprocess"}
// Dependencies: {}
# [doc = " A running subprocess for one of the sides of the benchmark (client or server)"] struct BenchSubprocess { # [doc = " The benchmark's child process, running under valgrind"] process : Child , # [doc = " Valgrind's output file for this benchmark"] output : ValgrindOutput , }
};
}
