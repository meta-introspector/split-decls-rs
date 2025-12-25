#!/bin/bash

# Missing dependencies from bootstrap log
missing_deps="toml anyhow serde proc-macro2"

echo "🔍 Wrapping missing dependencies: $missing_deps"

for dep in $missing_deps; do
    echo "📦 Processing $dep..."
    
    # Check if it's in the crates list in split-decls-rs.toml
    if grep -q "\"$dep\"" split-decls-rs.toml; then
        echo "  ✅ Found $dep in crates list"
        
        # Check for path override
        override_path=$(grep -A 1 "^$dep = " split-decls-rs.toml | grep -v "^$dep = " | sed 's/.*"\([^"]*\)".*/\1/')
        
        if [ -n "$override_path" ]; then
            echo "  📍 Using override path: $override_path"
            crate_path="$override_path"
        else
            # Try common locations
            if [ -d "../../submodules/$dep" ]; then
                crate_path="../../submodules/$dep"
            elif [ -d "../$dep" ]; then
                crate_path="../$dep"
            else
                echo "  ❌ Cannot find path for $dep"
                continue
            fi
        fi
        
        echo "  🚀 Wrapping $dep from $crate_path to output2..."
        cargo run --bin wrap_single_crate -- "$crate_path" --output output2 --verbose
        
    else
        echo "  ❌ $dep not found in split-decls-rs.toml crates list"
    fi
    echo ""
done

echo "✅ Dependency wrapping complete!"
