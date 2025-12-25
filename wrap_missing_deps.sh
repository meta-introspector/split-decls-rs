#!/bin/bash

# Extract missing dependencies from bootstrap log
echo "🔍 Extracting missing dependencies from bootstrap_run.log..."

missing_deps=$(grep "failed to load manifest for dependency" bootstrap_run.log | sed 's/.*dependency `\([^`]*\)`.*/\1/' | sort -u)

echo "Missing dependencies found:"
echo "$missing_deps"

# Check workspace Cargo.toml for these dependencies
workspace_cargo="../../Cargo.toml"

echo ""
echo "🔍 Checking workspace dependencies..."

for dep in $missing_deps; do
    echo "Checking for $dep..."
    
    # Look for the dependency in workspace dependencies
    dep_path=$(grep -A 5 "^$dep = " "$workspace_cargo" | grep "path = " | sed 's/.*path = "\([^"]*\)".*/\1/' | head -1)
    
    # Also check in [workspace.dependencies] section
    if [ -z "$dep_path" ]; then
        dep_path=$(awk '/^\[workspace\.dependencies\]/,/^\[/ {if ($0 ~ "^'$dep' = ") print}' "$workspace_cargo" | grep "path = " | sed 's/.*path = "\([^"]*\)".*/\1/' | head -1)
    fi
    
    if [ -n "$dep_path" ]; then
        echo "  ✅ Found $dep at path: $dep_path"
        
        # Convert relative path to absolute from workspace root
        full_path="../../$dep_path"
        
        if [ -d "$full_path" ]; then
            echo "  📦 Wrapping $dep from $full_path..."
            cargo run --bin wrap_single_crate -- "$full_path" --output output2 --verbose
        else
            echo "  ❌ Path not found: $full_path"
        fi
    else
        echo "  ❌ $dep not found in workspace dependencies"
    fi
    echo ""
done

echo "✅ Dependency wrapping complete!"
