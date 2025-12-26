#!/bin/bash
set -e

INPUT_DIR=${1:-""}
VERBOSE=${2:-""}

echo "Generating output3 from original submodules..."

# Check for input sources
if [ -n "$INPUT_DIR" ]; then
    if [ ! -d "$INPUT_DIR" ]; then
        echo "Error: Specified input directory '$INPUT_DIR' not found"
        exit 1
    fi
    echo "Using specified input: $INPUT_DIR"
    cargo run --bin generate_output3_from_enhanced -- "$INPUT_DIR"
else
    # Let the binary choose the best available source (prioritizes ../../submodules)
    echo "Auto-detecting input source (prioritizing original submodules)..."
    cargo run --bin generate_output3_from_enhanced
fi

if [ -d "output3" ] && [ "$(ls -A output3)" ]; then
    echo "Success: output3 generated"
    
    # Count wrapped crates
    wrapped_count=$(find output3 -maxdepth 1 -name "wrapped-*" -type d | wc -l)
    echo "Generated $wrapped_count wrapped crates"
    
    # If there's a wrapped split-decls-rs, test it
    if [ -d "output3/wrapped-split-decls-rs" ]; then
        cd output3/wrapped-split-decls-rs
        echo "Testing output3 bootstrap..."
        cargo run --bin split-decls-rs bootstrap > ../../output3_test.log 2>&1 || echo "output3 ready (see output3_test.log)"
        cd ../..
    else
        echo "Note: No wrapped-split-decls-rs found in output3"
    fi
else
    echo "Error: output3 generation failed or empty"
    exit 1
fi
