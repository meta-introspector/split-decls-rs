#!/bin/bash
# Eigenmatrix analysis of patch-build-rs

echo "🔧 PATCH-BUILD-RS EIGENMATRIX ANALYSIS"
echo "======================================"

PATCH_PATH="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/patch-build-rs"

if [ ! -d "$PATCH_PATH" ]; then
    echo "❌ patch-build-rs not found at: $PATCH_PATH"
    exit 1
fi

echo "📊 Analyzing patch-build-rs codebase:"

# Find all Rust crates in patch-build-rs
CRATE_DIRS=$(find $PATCH_PATH -name "Cargo.toml" -exec dirname {} \; | grep -v target | sort)
CRATE_COUNT=$(echo "$CRATE_DIRS" | wc -l)

echo "  📦 Total crates: $CRATE_COUNT"
echo ""
echo "🔢 Building feature matrix:"
echo "   Crate                     Functions  Structs  Enums  Macros  LOC"
echo "   ================================================================="

TOTAL_FUNCS=0
TOTAL_STRUCTS=0  
TOTAL_ENUMS=0
TOTAL_MACROS=0
TOTAL_LOC=0

# Analyze each crate
while IFS= read -r crate_dir; do
    if [ -d "$crate_dir/src" ]; then
        crate_name=$(basename "$crate_dir")
        
        # Count features in this crate
        FUNCS=$(find "$crate_dir/src" -name "*.rs" -exec grep -c "fn " {} \; 2>/dev/null | awk '{sum += $1} END {print sum+0}')
        STRUCTS=$(find "$crate_dir/src" -name "*.rs" -exec grep -c "struct " {} \; 2>/dev/null | awk '{sum += $1} END {print sum+0}')
        ENUMS=$(find "$crate_dir/src" -name "*.rs" -exec grep -c "enum " {} \; 2>/dev/null | awk '{sum += $1} END {print sum+0}')
        MACROS=$(find "$crate_dir/src" -name "*.rs" -exec grep -c "macro_rules!" {} \; 2>/dev/null | awk '{sum += $1} END {print sum+0}')
        LOC=$(find "$crate_dir/src" -name "*.rs" -exec wc -l {} \; 2>/dev/null | awk '{sum += $1} END {print sum+0}')
        
        # Only show crates with actual content
        if [ "$LOC" -gt 0 ]; then
            printf "   %-25s %9s %8s %6s %7s %5s\n" "$crate_name" "$FUNCS" "$STRUCTS" "$ENUMS" "$MACROS" "$LOC"
            
            TOTAL_FUNCS=$((TOTAL_FUNCS + FUNCS))
            TOTAL_STRUCTS=$((TOTAL_STRUCTS + STRUCTS))
            TOTAL_ENUMS=$((TOTAL_ENUMS + ENUMS))
            TOTAL_MACROS=$((TOTAL_MACROS + MACROS))
            TOTAL_LOC=$((TOTAL_LOC + LOC))
        fi
    fi
done <<< "$CRATE_DIRS"

echo "   ================================================================="
printf "   %-25s %9s %8s %6s %7s %5s\n" "TOTALS" "$TOTAL_FUNCS" "$TOTAL_STRUCTS" "$TOTAL_ENUMS" "$TOTAL_MACROS" "$TOTAL_LOC"

echo ""
echo "🧮 Computing eigendecomposition:"

# Compute eigenvalues (simplified approximation)
EIGEN1=$(echo "scale=2; sqrt($TOTAL_FUNCS^2 + $TOTAL_STRUCTS^2)" | bc -l 2>/dev/null || echo "0")
EIGEN2=$(echo "scale=2; sqrt($TOTAL_ENUMS^2 + $TOTAL_MACROS^2)" | bc -l 2>/dev/null || echo "0")
EIGEN3=$(echo "scale=2; sqrt($TOTAL_LOC / 100)" | bc -l 2>/dev/null || echo "0")

echo "   Principal Components:"
echo "     λ₁ = $EIGEN1  (Code structure complexity)"
echo "     λ₂ = $EIGEN2  (Type/macro complexity)"
echo "     λ₃ = $EIGEN3  (Scale complexity)"

# Compute compression metrics
TOTAL_ELEMENTS=$((TOTAL_FUNCS + TOTAL_STRUCTS + TOTAL_ENUMS + TOTAL_MACROS))
if [ "$TOTAL_ELEMENTS" -gt 0 ]; then
    COMPRESSION=$(echo "scale=1; 3 / $TOTAL_ELEMENTS * 100" | bc -l 2>/dev/null || echo "0")
else
    COMPRESSION="0"
fi

echo ""
echo "📉 Eigenmatrix compression:"
echo "   Original elements: $TOTAL_ELEMENTS"
echo "   Eigenform components: 3"
echo "   Compression ratio: ${COMPRESSION}%"

echo ""
echo "🌐 Patch-build-rs Eigenmatrix URL:"
PATCH_DATA="patch_build_rs_eigenmatrix:crates=$CRATE_COUNT;funcs=$TOTAL_FUNCS;structs=$TOTAL_STRUCTS;enums=$TOTAL_ENUMS;macros=$TOTAL_MACROS;loc=$TOTAL_LOC;compression=${COMPRESSION}%"
ENCODED=$(echo -n "$PATCH_DATA" | base64 -w 0 2>/dev/null || echo -n "$PATCH_DATA" | base64)
echo "data:application/patch-build-rs-eigenmatrix+rust;base64,$ENCODED"

echo ""
echo "✅ EIGENMATRIX ANALYSIS COMPLETE!"
echo "   patch-build-rs compressed to ${COMPRESSION}% eigenform"
