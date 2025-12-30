#!/bin/bash

# Research script to analyze each missing rustc term systematically
# Based on missing_content_report.json

echo "🔍 Systematic Term Research"
echo "=========================="

# Missing terms from the report
TERMS=(
    "get_resident_set_size"
    "TimePassesCallbacks" 
    "EarlyDiagCtxt"
    "ErrorOutputType"
    "init_rustc_env_logger"
    "install_ice_hook"
    "install_ctrlc_handler"
    "run_compiler"
    "catch_with_exit_code"
    "print_time_passes_entry"
    "DEFAULT_BUG_REPORT_URL"
)

echo "📊 Analyzing ${#TERMS[@]} critical rustc terms..."
echo ""

for i in "${!TERMS[@]}"; do
    term="${TERMS[$i]}"
    echo "🔍 [$((i+1))/${#TERMS[@]}] Researching: $term"
    
    # 1. Direct grep in output2
    echo "  📁 Direct file search:"
    files=$(grep -r "$term" output2/ 2>/dev/null | wc -l)
    if [ "$files" -gt 0 ]; then
        echo "    ✅ Found in $files locations"
        grep -r "$term" output2/ 2>/dev/null | head -2 | sed 's/^/      /'
    else
        echo "    ❌ Not found in direct search"
    fi
    
    # 2. Check comprehensive index
    echo "  📊 Comprehensive index lookup:"
    if [ -f "comprehensive_syn_index.json" ]; then
        if jq -e ".token_classifications[\"$term\"]" comprehensive_syn_index.json >/dev/null 2>&1; then
            occurrences=$(jq -r ".token_classifications[\"$term\"].occurrences" comprehensive_syn_index.json)
            classification=$(jq -r ".token_classifications[\"$term\"].classification" comprehensive_syn_index.json)
            echo "    ✅ Found: $occurrences occurrences, type: $classification"
        else
            echo "    ❌ Not in comprehensive index"
        fi
    else
        echo "    ⚠️  Comprehensive index not available"
    fi
    
    # 3. Check fast lookup
    echo "  ⚡ Fast lookup check:"
    if [ -f "fast_token_lookup.json" ]; then
        if jq -e ".[\"$term\"]" fast_token_lookup.json >/dev/null 2>&1; then
            location=$(jq -r ".[\"$term\"]" fast_token_lookup.json)
            echo "    ✅ Fast lookup: $location"
        else
            echo "    ❌ Not in fast lookup"
        fi
    else
        echo "    ⚠️  Fast lookup not available"
    fi
    
    # 4. Pattern variations
    echo "  🔄 Pattern variations:"
    # Try snake_case to CamelCase
    camel_case=$(echo "$term" | sed 's/_\([a-z]\)/\U\1/g' | sed 's/^./\U&/')
    if [ "$camel_case" != "$term" ]; then
        camel_files=$(grep -r "$camel_case" output2/ 2>/dev/null | wc -l)
        if [ "$camel_files" -gt 0 ]; then
            echo "    ✅ CamelCase variant '$camel_case' found in $camel_files locations"
        fi
    fi
    
    # Try partial matches
    partial=$(echo "$term" | cut -c1-10)
    partial_files=$(grep -r "$partial" output2/ 2>/dev/null | wc -l)
    if [ "$partial_files" -gt 0 ] && [ "$partial_files" -lt 20 ]; then
        echo "    🔍 Partial match '$partial*' found in $partial_files locations"
    fi
    
    echo ""
done

echo "📋 Research Summary"
echo "=================="
echo "✅ Use this data to improve the name indexer"
echo "✅ Focus on terms found in direct search but missing from index"
echo "✅ Check why syn parsing missed these tokens"
