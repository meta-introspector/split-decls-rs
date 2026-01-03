#!/bin/bash

# Quick Fix Automation Script
# Run this after getting actionable errors from unified-build

echo "🔧 QUICK FIX AUTOMATION"
echo "======================="

# Function to fix attribute spacing
fix_attribute_spacing() {
    echo "🔄 Fixing attribute spacing..."
    find layer1-unified-build/output/processed -name "*.rs" -exec sed -i 's/# \[/#[/g' {} \;
    find layer1-unified-build/output/processed -name "*.rs" -exec sed -i 's/# \!/#!/g' {} \;
    echo "✅ Attribute spacing fixed"
}

# Function to add common macro stubs
add_macro_stubs() {
    echo "🔄 Adding common macro stubs..."
    cat >> layer1-unified-build/src/macro_stubs.rs << 'EOF'
// Auto-generated macro stubs
macro_rules! mkitem { ($($tt:tt)*) => { $($tt)* }; }
macro_rules! mkfn { ($($tt:tt)*) => { $($tt)* }; }
macro_rules! mkmod { ($($tt:tt)*) => { $($tt)* }; }
macro_rules! mkuse { ($($tt:tt)*) => { $($tt)* }; }
EOF
    echo "✅ Macro stubs added"
}

# Function to add common import stubs
add_import_stubs() {
    echo "🔄 Adding common import stubs..."
    cat >> layer1-unified-build/src/import_stubs.rs << 'EOF'
// Auto-generated import stubs
pub use std::collections::*;
pub use std::sync::*;
pub use std::thread::*;
EOF
    echo "✅ Import stubs added"
}

# Parse command line arguments
case "$1" in
    "attributes")
        fix_attribute_spacing
        ;;
    "macros")
        add_macro_stubs
        ;;
    "imports")
        add_import_stubs
        ;;
    "all")
        fix_attribute_spacing
        add_macro_stubs
        add_import_stubs
        ;;
    *)
        echo "Usage: $0 {attributes|macros|imports|all}"
        echo ""
        echo "  attributes - Fix # [attr] → #[attr] spacing"
        echo "  macros     - Add common macro stubs"
        echo "  imports    - Add common import stubs"
        echo "  all        - Apply all fixes"
        exit 1
        ;;
esac

echo ""
echo "🎉 Quick fixes applied! Run unified-build again to test."
