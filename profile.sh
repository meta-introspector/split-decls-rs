#!/bin/bash

# Basic profiling script for split-decls-rs

echo "=== CPU Profiling with Flamegraph ==="
cargo flamegraph --bin split-decls-rs -- bootstrap

echo "=== Memory Profiling with Valgrind (if available) ==="
if command -v valgrind &> /dev/null; then
    cargo build --release
    valgrind --tool=massif --massif-out-file=massif.out ./target/release/split-decls-rs bootstrap
    ms_print massif.out > memory_profile.txt
    echo "Memory profile saved to memory_profile.txt"
fi

echo "=== Time Profiling ==="
time cargo run --release --bin split-decls-rs -- bootstrap

echo "=== Perf Profiling (Linux) ==="
if command -v perf &> /dev/null; then
    cargo build --release
    perf record -g ./target/release/split-decls-rs bootstrap
    perf report > perf_report.txt
    echo "Perf report saved to perf_report.txt"
fi
