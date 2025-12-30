// Generated macro for print_bench_results (function)
macro_rules! Depcrateprint_bench_results {
() => {
// Module: crate
// Provides: {"print_bench_results"}
// Dependencies: {}
fn print_bench_results (res : & BenchRunResults) { let allocation_success = (res . successful_allocations as f64 / res . allocation_attempts as f64) * 100.0 ; log_benchmark_data ("Allocation success" , "%" , allocation_success) ; let deallocation_success = (res . deallocations as f64 / res . successful_allocations as f64) * 100.0 ; log_benchmark_data ("Deallocation success" , "%" , deallocation_success) ; let pre_fail_alloc = (res . pre_fail_allocations as f64 / res . allocation_attempts as f64) * 100.0 ; log_benchmark_data ("Pre-fail Allocations" , "%" , pre_fail_alloc) ; let avg_all_alloc = res . all_alloc_measurements . iter () . sum :: < u64 > () as f64 / res . all_alloc_measurements . len () as f64 ; log_benchmark_data ("Average Allocation time" , "Ticks" , avg_all_alloc) ; let avg_nofail_alloc = res . nofail_alloc_measurements . iter () . sum :: < u64 > () as f64 / res . nofail_alloc_measurements . len () as f64 ; log_benchmark_data ("Average Allocation time (no fail)" , "Ticks" , avg_nofail_alloc ,) ; let avg_dealloc = res . dealloc_measurements . iter () . sum :: < u64 > () as f64 / res . dealloc_measurements . len () as f64 ; log_benchmark_data ("Average Deallocation time" , "Ticks" , avg_dealloc) ; }
};
}
