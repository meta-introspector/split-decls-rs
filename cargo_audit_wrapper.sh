#!/bin/bash

# Audit wrapper for cargo run commands
echo "🔍 AUDIT: Executing cargo command at $(date)"
echo "📋 Command: cargo $*"
echo "📁 Working directory: $(pwd)"

start_time=$(date +%s.%N)

# Execute the actual cargo command
cargo "$@"
exit_code=$?

end_time=$(date +%s.%N)
duration=$(echo "$end_time - $start_time" | bc -l)

if [ $exit_code -eq 0 ]; then
    echo "✅ SUCCESS: Cargo command completed in ${duration}s"
    echo "📤 Exit code: $exit_code"
else
    echo "❌ ERROR: Cargo command failed in ${duration}s"
    echo "📤 Exit code: $exit_code"
fi

echo "----------------------------------------"
exit $exit_code
