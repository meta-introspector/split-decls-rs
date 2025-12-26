#!/bin/bash

echo "🔍 syn-mold: Static Analysis Mold System Demo"
echo "=============================================="

echo ""
echo "📋 Testing syn-mold on heavy syn usage code..."

# Build the tools first
echo "🔨 Building syn-mold tools..."
cargo build --bin syn_mold --bin syn2macro

echo ""
echo "🧬 Analyzing test_syn_heavy.rs for syn usage patterns..."
cargo run --bin syn_mold -- -i test_syn_heavy.rs -a -o test_syn_heavy_molded.rs

echo ""
echo "🔄 Generating replacement code with compile-time checking..."
cargo run --bin syn_mold -- -i test_syn_heavy.rs -r -o test_syn_heavy_replaced.rs

echo ""
echo "📊 Generated files:"
ls -la test_syn_heavy*.rs

echo ""
echo "🎯 syn2macro universal trait conversion..."
cargo run --bin syn2macro -- -i test_input.rs -o test_input_traits.rs -c abstract -s strict

echo ""
echo "✅ Demo complete! Check the generated files:"
echo "  - test_syn_heavy_molded.rs: Mold wrapper for syn usage"
echo "  - test_syn_heavy_replaced.rs: Replacement code with compile-time checks"
echo "  - test_syn_heavy_mold_report.rs: Static analysis report"
echo "  - test_input_traits.rs: Universal trait-based version"
echo "  - test_input_example.rs: Usage example"

echo ""
echo "🧠 Key innovations:"
echo "  ✓ Static analysis of syn usage complexity"
echo "  ✓ Compile-time signature extraction"
echo "  ✓ Inside-out wrapper generation"
echo "  ✓ Multi-context execution (compiler/syn/abstract)"
echo "  ✓ Security and ACL integration"
echo "  ✓ Complexity threshold validation"
