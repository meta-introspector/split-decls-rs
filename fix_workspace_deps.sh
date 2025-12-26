#!/bin/bash
# Quick fix for workspace dependency issues

echo "Fixing workspace dependencies in wrapped crates..."

cd output2

# Fix wrapped-reson
if [ -f "wrapped-reson/Cargo.toml" ]; then
    echo "Fixing wrapped-reson..."
    sed -i 's/workspace = true/version = "*"/g' wrapped-reson/Cargo.toml
fi

# Fix wrapped-pagerank_rs  
if [ -f "wrapped-pagerank_rs/Cargo.toml" ]; then
    echo "Fixing wrapped-pagerank_rs..."
    sed -i 's/workspace = true/version = "*"/g' wrapped-pagerank_rs/Cargo.toml
fi

echo "✅ Fixed workspace dependencies"
