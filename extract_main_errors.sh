#!/bin/bash
# Extract errors from main build.log and feed to incremental compiler

echo "🔍 Extracting errors from main build..."

# Extract unique error patterns from build.log
grep "error\[E[0-9]\+\]:" build.log | \
    sed 's/.*error\[E[0-9]\+\]: //' | \
    sort | uniq -c | sort -nr > main_build_errors.txt

echo "📊 Top 10 error types from main build:"
head -10 main_build_errors.txt

# Extract files that failed compilation
echo "📁 Extracting failed source files..."
grep -B5 "error\[E[0-9]\+\]:" build.log | \
    grep "src/" | \
    sed 's/.*src\//src\//' | \
    sed 's/:.*$//' | \
    sort | uniq > failed_files.txt

echo "📋 Found $(wc -l < failed_files.txt) unique files with errors"

# Create a focused test with these files
echo "🎯 Testing incremental compiler on failed files..."
cd ../incremental-rust-compiler

# Test with first 10 failed files
head -10 ../split-decls-genesis/failed_files.txt | while read file; do
    echo "Testing: $file"
    # Convert to full path format expected by incremental compiler
    full_path="../rust/$(echo $file | sed 's/^src\///')"
    echo "$full_path" >> test_failed_files.txt
done

echo "🔧 Running incremental compiler on main build failures..."
cargo run --bin genesis_incremental_driver 10
