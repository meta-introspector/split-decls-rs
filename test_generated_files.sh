#!/bin/bash

echo "🚀 Testing Layer 1 generated files - FAIL FAST MODE"

PROCESSED_DIR="layer1-unified-build/output/processed"

if [ ! -d "$PROCESSED_DIR" ]; then
    echo "❌ No processed files found at $PROCESSED_DIR"
    exit 1
fi

echo "📁 Testing files in: $PROCESSED_DIR"

# Test first file that fails
find "$PROCESSED_DIR" -name "*.rs" | head -10 | while read -r file; do
    filename=$(basename "$file")
    
    printf "Testing %-60s ... " "$filename"
    
    # Try to parse with rustc
    if rustc --crate-type lib --allow warnings "$file" -o /tmp/test_output 2>/tmp/error.log; then
        echo "✅ OK"
        rm -f /tmp/test_output
    else
        echo "❌ FAIL"
        echo ""
        echo "🔍 FIRST FAILURE DIAGNOSIS:"
        echo "📁 File: $file"
        echo "📄 Content preview:"
        head -20 "$file"
        echo ""
        echo "❌ Error details:"
        cat /tmp/error.log
        echo ""
        echo "🔧 STOPPING ON FIRST ERROR FOR DIAGNOSIS"
        exit 1
    fi
done
