#!/bin/bash

echo "🧪 Quick Single File Test"

# Find the smallest processed file
SMALLEST=$(ls -la src/processed_*.rs | sort -k5 -n | head -1 | awk '{print $9}')
echo "Testing smallest file: $SMALLEST"

# Try to compile just this one file
rustc --crate-type lib --allow warnings --edition 2021 "$SMALLEST" -o /tmp/test.rlib 2>&1 | head -20

echo "✅ Quick test complete"
