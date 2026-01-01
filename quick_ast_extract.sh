#!/bin/bash

echo "🔍 Quick AST Error Extraction"

# Extract error files and line numbers
grep "error\[E" build.log | grep "src/processed_" | head -10 | while read line; do
    file=$(echo "$line" | sed 's/.*--> \(src\/[^:]*\).*/\1/')
    error=$(echo "$line" | sed 's/.*error\[\([^]]*\)\].*/\1/')
    echo "📁 $file - Error: $error"
    
    # Show AST metadata from that file
    if [ -f "$file" ]; then
        grep "AST_META:" "$file" | head -2
        echo "---"
    fi
done

echo "✅ Done"
