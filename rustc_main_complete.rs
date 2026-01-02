// Auto-generated rustc main with all dependencies

include!("processed_.._rust_compiler_rustc_data_structures_src_profiling.rs"); // rustc_data_structures::profiling::print_time_passes_entry
include!("processed_.._rust_compiler_rustc_driver_impl_src_lib.rs"); // rustc_driver_impl::lib::main
// Define the include_dep! macro
macro_rules! include_dep {
    ($dep:expr, $file:expr) => {
        println!("Loading dependency: {}", $dep);
        include!($file);
    };
}


macro_rules! rustcmain {
    () => {
        include_dep!("rustc_data_structures::profiling::print_time_passes_entry", "processed_.._rust_compiler_rustc_data_structures_src_profiling.rs");
        include_dep!("rustc_driver_impl::lib::main", "processed_.._rust_compiler_rustc_driver_impl_src_lib.rs");
    };
}

fn main() {
    rustcmain!();
}
