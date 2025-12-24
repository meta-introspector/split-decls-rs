use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A wrapper around google_cpu_profiler.
///
/// Usage:
/// 1. Install gperf_tools (<https://github.com/gperftools/gperftools>), probably packaged with your Linux distro.
/// 2. Build with `cpu_profiler` feature.
/// 3. Run the code, the *raw* output would be in the `./out.profile` file.
/// 4. Install pprof for visualization (<https://github.com/google/pprof>).
/// 5. Bump sampling frequency to once per ms: `export CPUPROFILE_FREQUENCY=1000`
/// 6. Use something like `pprof -svg target/release/rust-analyzer ./out.profile` to see the results.
///
/// For example, here's how I run profiling on NixOS:
///
/// ```bash
/// $ bat -p shell.nix
/// with import <nixpkgs> {};
/// mkShell {
///   buildInputs = [ gperftools ];
///   shellHook = ''
///     export LD_LIBRARY_PATH="${gperftools}/lib:"
///   '';
/// }
/// $ set -x CPUPROFILE_FREQUENCY 1000
/// $ nix-shell --run 'cargo test --release --package rust-analyzer --lib -- benchmarks::benchmark_integrated_highlighting --exact --nocapture'
/// $ pprof -svg target/release/deps/rust_analyzer-8739592dc93d63cb crates/rust-analyzer/out.profile > profile.svg
/// ```
///
/// See this diff for how to profile completions:
///
/// <https://github.com/rust-lang/rust-analyzer/pull/5306>
#[derive(Debug)]
pub struct CpuSpan {
    _private: (),
}
