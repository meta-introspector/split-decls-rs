// Generated macro for impl_67 (impl)
macro_rules! Depcrate_valgrindimpl_67 {
() => {
// Module: crate::valgrind
// Provides: {"impl_67"}
// Dependencies: {}
impl BenchSubprocess { # [doc = " Waits for the process to finish and returns the measured instruction count"] fn wait_and_get_instr_count (mut self) -> anyhow :: Result < u64 > { let status = self . process . wait () . context ("Failed to run benchmark in callgrind") ? ; if ! status . success () { anyhow :: bail ! ("Failed to run benchmark in callgrind. Exit code: {:?}" , status . code ()) ; } let ValgrindOutput :: Callgrind { output_file } = self . output else { panic ! ("wait_and_get_instr_count() is for Callgrind users") ; } ; parse_callgrind_output (& output_file) } # [doc = " Waits for the process to finish and returns the measured peak heap usage"] fn wait_and_get_memory_details (mut self) -> anyhow :: Result < MemoryDetails > { let status = self . process . wait () . context ("Failed to run benchmark in DHAT") ? ; if ! status . success () { anyhow :: bail ! ("Failed to run benchmark in DHAT. Exit code: {:?}" , status . code ()) ; } let ValgrindOutput :: Dhat { log_file } = self . output else { panic ! ("wait_and_get_memory_details() is for DHAT users") ; } ; MemoryDetails :: from_file (& log_file) } }
};
}
