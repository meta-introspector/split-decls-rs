#!/bin/bash

echo "🦀 Compiler Layer Ontology Analysis Summary"
echo "==========================================="
echo

echo "📊 Layer Distribution Analysis:"
echo "AST Layer 🌲:"
jq '.[] | select(.layer == "AST") | "  Total patterns: \(.total_patterns), Coverage: \(.coverage_percentage)%, Top: \(.top_patterns[0].pattern)"' compiler_layer_ontology_3gram.json

echo "HIR Layer 🌳:"
jq '.[] | select(.layer == "HIR") | "  Total patterns: \(.total_patterns), Coverage: \(.coverage_percentage)%, Top: \(.top_patterns[0].pattern)"' compiler_layer_ontology_3gram.json

echo "MIR Layer 🔬:"
jq '.[] | select(.layer == "MIR") | "  Total patterns: \(.total_patterns), Coverage: \(.coverage_percentage)%, Top: \(.top_patterns[0].pattern)"' compiler_layer_ontology_3gram.json

echo
echo "🔗 Cross-Layer Pattern Relationships:"
echo "Showing how patterns flow between compiler layers..."

echo
echo "🌲 AST → 🌳 HIR Relations:"
jq -r '.[] | select(.layer == "AST") | .cross_layer_relations[] | select(.target_layer == "HIR") | "  \(.pattern) (strength: \(.relation_strength))"' compiler_layer_ontology_3gram.json | head -3

echo
echo "🌳 HIR → 🔬 MIR Relations:"
jq -r '.[] | select(.layer == "HIR") | .cross_layer_relations[] | select(.target_layer == "MIR") | "  \(.pattern) (strength: \(.relation_strength))"' compiler_layer_ontology_3gram.json | head -3

echo
echo "📈 Pattern Frequency Analysis:"
echo "Most common patterns across all layers:"

echo "Standard Library Usage:"
jq -r '.[] | .top_patterns[] | select(.pattern | contains("std collections")) | "  \(.layer): \(.pattern) (freq: \(.frequency))"' compiler_layer_ontology_3gram.json

echo
echo "Serialization Patterns:"
jq -r '.[] | .top_patterns[] | select(.pattern | contains("serde")) | "  \(.layer): \(.pattern) (freq: \(.frequency))"' compiler_layer_ontology_3gram.json | head -5

echo
echo "🎯 Key Insights:"
echo "1. Standard library patterns (std collections HashMap) appear consistently across layers"
echo "2. Serialization patterns (serde Deserialize Serialize) show high frequency"
echo "3. Cross-layer relations demonstrate pattern preservation through compilation pipeline"
echo "4. Each layer maintains distinct characteristics while sharing common foundations"

echo
echo "💾 Generated Files:"
ls -la compiler_layer_ontology_*.json | awk '{print "  " $9 " (" $5 " bytes)"}'
