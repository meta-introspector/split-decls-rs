#!/bin/bash
# Proof system to audit all generated code for syntax errors

echo "🔍 PROOF SYSTEM: Auditing Generated Code"
echo "========================================"

# Check for unclosed delimiters
echo "📊 Checking for unclosed delimiters..."
grep -n "unclosed delimiter" build.log | head -5

# Find the problematic file
echo "📁 Locating problematic generated code..."
grep -B5 -A5 "included_rustc_trait_selection" src/lib.rs | head -20

# Count braces in generated modules
echo "🔧 Brace balance check..."
for file in src/lib.rs src/lib_incremental.rs; do
    if [ -f "$file" ]; then
        open_braces=$(grep -o '{' "$file" | wc -l)
        close_braces=$(grep -o '}' "$file" | wc -l)
        echo "$file: { = $open_braces, } = $close_braces, diff = $((open_braces - close_braces))"
    fi
done

# Check for incomplete module declarations
echo "📋 Checking module declarations..."
grep -n "pub mod.*{$" src/lib.rs | tail -5

echo "✅ Proof audit complete"
