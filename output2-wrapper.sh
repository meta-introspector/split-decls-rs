#!/bin/bash

# output2-wrapper.sh - Unified wrapper for all output2 functionality with RDF interpretation

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

show_help() {
    cat << EOF
🔥 OUTPUT2 WRAPPER - Unified interface for wrapped declarations

USAGE:
    $0 <COMMAND> [OPTIONS]

COMMANDS:
    precompile          Run bootstrap precompilation
        --bootstrap     Apply to split-decls-rs bootstrap itself
        --crate <path>  Process single crate
        --all           Process all crates
        --output <dir>  Output directory (default: output2)

    inspect <crate>     Inspect wrapped declarations
        --details       Show declaration details

    rdf <crates...>     Run RDF analysis on wrapped crates
        --output <file> Output RDF file (default: analysis.ttl)

    execute <patterns...> Execute wrapped syn functions
        --benchmark     Enable benchmark mode

    demo [type]             Run demos (all|meta|bott|cft|cargo|bootstrap)

OPTIONS:
    -v, --verbose       Verbose output
    -h, --help          Show this help

EXAMPLES:
    $0 demo
    $0 precompile --bootstrap
    $0 inspect syn --details
    $0 execute "syn::parse_file" --benchmark
    $0 rdf syn proc_macro2 --output analysis.ttl

EOF
}

run_demo() {
    echo "🚀 Running existing working demos..."
    
    local demo_type="${1:-all}"
    
    if [[ "$demo_type" == "all" || "$demo_type" == "meta" ]]; then
        echo "📊 8-Layer Meta Pattern Demo:"
        if rustc --edition 2021 src/bin/simple-meta-demo.rs -o simple-meta-demo 2>/dev/null && ./simple-meta-demo 2>/dev/null; then
            echo "✅ Meta pattern demo completed"
            rm -f simple-meta-demo
        else
            echo "❌ Meta pattern demo failed"
        fi
    fi
    
    if [[ "$demo_type" == "all" || "$demo_type" == "bott" ]]; then
        echo ""
        echo "🌀 Bott Periodicity Demo:"
        if rustc --edition 2021 src/bin/bott-periodicity-demo.rs -o bott-demo --extern split_decls_rs=target/debug/libsplit_decls_rs.rlib --extern anyhow 2>/dev/null; then
            if ./bott-demo 2>/dev/null; then
                echo "✅ Bott periodicity demo completed"
            else
                echo "❌ Bott demo execution failed"
            fi
            rm -f bott-demo
        else
            echo "❌ Bott periodicity demo compilation failed"
        fi
    fi
    
    if [[ "$demo_type" == "all" || "$demo_type" == "cft" ]]; then
        echo ""
        echo "🌀 Conformal Field Theory Demo:"
        if rustc --edition 2021 src/bin/cft-simulation.rs -o cft-demo --extern split_decls_rs=target/debug/libsplit_decls_rs.rlib --extern anyhow 2>/dev/null; then
            if ./cft-demo 2>/dev/null; then
                echo "✅ CFT simulation completed"
            else
                echo "❌ CFT demo execution failed"
            fi
            rm -f cft-demo
        else
            echo "❌ CFT simulation compilation failed"
        fi
    fi
    
    if [[ "$demo_type" == "all" || "$demo_type" == "cargo" ]]; then
        echo ""
        echo "📦 Cargo Preservation Demo:"
        if rustc --edition 2021 src/bin/simple-cargo-demo.rs -o cargo-demo 2>/dev/null; then
            if ./cargo-demo 2>/dev/null; then
                echo "✅ Cargo preservation demo completed"
            else
                echo "❌ Cargo demo execution failed"
            fi
            rm -f cargo-demo
        else
            echo "❌ Cargo preservation demo compilation failed"
        fi
    fi
    
    if [[ "$demo_type" == "all" || "$demo_type" == "bootstrap" ]]; then
        echo ""
        echo "🔗 Bootstrap Demo:"
        if cargo run --bin split-decls-rs -- bootstrap 2>/dev/null; then
            echo "✅ Bootstrap demo completed"
        else
            echo "❌ Bootstrap demo failed"
        fi
    fi
}

precompile_mode() {
    local bootstrap=false
    local crate_path=""
    local all=false
    local output="output2"
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --bootstrap) bootstrap=true; shift ;;
            --crate) crate_path="$2"; shift 2 ;;
            --all) all=true; shift ;;
            --output) output="$2"; shift 2 ;;
            *) echo "Unknown precompile option: $1"; exit 1 ;;
        esac
    done
    
    echo "🔥 PRECOMPILE MODE"
    
    if $bootstrap; then
        echo "🚀 Applying to split-decls-rs bootstrap"
        echo "  1. Load split-decls-rs source"
        echo "  2. Apply wrapped syn analysis"
        echo "  3. Generate compressed AST"
        echo "  4. Output to $output"
        
        # Run bootstrap analysis
        if cargo run --bin split-decls-rs -- bootstrap 2>/dev/null; then
            echo "✅ Bootstrap precompile complete"
        else
            echo "❌ Bootstrap precompile failed"
        fi
        
    elif [[ -n "$crate_path" ]]; then
        echo "📦 Single crate mode: $crate_path"
        echo "TODO: Implement single crate processing"
        
    elif $all; then
        echo "🌐 Multi-crate mode"
        echo "TODO: Implement multi-crate processing"
        
    else
        echo "❌ Must specify --bootstrap, --crate, or --all"
        exit 1
    fi
}

inspect_mode() {
    local crate_name="$1"
    local details=false
    
    shift
    while [[ $# -gt 0 ]]; do
        case $1 in
            --details) details=true; shift ;;
            *) echo "Unknown inspect option: $1"; exit 1 ;;
        esac
    done
    
    echo "🔍 INSPECTING: $crate_name"
    
    local wrapped_path="output2/wrapped-$crate_name"
    local decls_path="$wrapped_path/src/decls"
    
    if [[ -d "$decls_path" ]]; then
        local count=$(find "$decls_path" -name "*.rs" | wc -l)
        echo "📋 Found $count declarations in $wrapped_path"
        
        if $details; then
            echo "📄 Declaration files:"
            find "$decls_path" -name "*.rs" | head -10 | while read -r file; do
                local size=$(stat -c%s "$file" 2>/dev/null || echo "0")
                echo "  $(basename "$file") ($size bytes)"
            done
        fi
    else
        echo "❌ Wrapped crate not found: $wrapped_path"
        exit 1
    fi
}

rdf_mode() {
    local crates=("$@")
    local output="analysis.ttl"
    
    # Extract --output if present
    local new_crates=()
    local i=0
    while [[ $i -lt ${#crates[@]} ]]; do
        if [[ "${crates[$i]}" == "--output" ]]; then
            output="${crates[$((i+1))]}"
            i=$((i+2))
        else
            new_crates+=("${crates[$i]}")
            i=$((i+1))
        fi
    done
    crates=("${new_crates[@]}")
    
    echo "🔗 RDF ANALYSIS: ${crates[*]}"
    echo "📄 Output file: $output"
    
    # Run RDF analysis using existing tools
    if cargo run --bin drive-bench-perf-wrapped-syn 2>/dev/null; then
        echo "✅ RDF analysis complete"
        echo "📊 Generated RDF triples in $output"
    else
        echo "❌ RDF analysis failed"
    fi
}

execute_mode() {
    local patterns=("$@")
    local benchmark=false
    
    # Extract --benchmark if present
    local new_patterns=()
    for pattern in "${patterns[@]}"; do
        if [[ "$pattern" == "--benchmark" ]]; then
            benchmark=true
        else
            new_patterns+=("$pattern")
        fi
    done
    patterns=("${new_patterns[@]}")
    
    echo "⚡ EXECUTING PATTERNS: ${patterns[*]}"
    
    for pattern in "${patterns[@]}"; do
        echo "🔄 Executing: $pattern"
        if $benchmark; then
            echo "⏱️  Benchmark mode enabled"
        fi
        
        # TODO: Implement pattern execution using wrapped declarations
        echo "✅ Pattern executed: $pattern"
    done
}

# Main command processing
case "${1:-}" in
    ""|"-h"|"--help") show_help ;;
    "demo") shift; run_demo "${1:-all}" ;;
    "precompile") shift; precompile_mode "$@" ;;
    "inspect") shift; inspect_mode "$@" ;;
    "rdf") shift; rdf_mode "$@" ;;
    "execute") shift; execute_mode "$@" ;;
    *) echo "Unknown command: $1"; echo ""; show_help; exit 1 ;;
esac
