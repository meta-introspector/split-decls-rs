#!/bin/bash

echo "🚀 RDF Knowledge Base Query Demo"
echo "================================="
echo

echo "📊 Basic Statistics:"
echo "stats" | cargo run --bin rdf_cli --quiet 2>/dev/null | grep -A5 "Knowledge Base Statistics"

echo
echo "🔍 Sample Queries:"

echo "query dependsOn" | cargo run --bin rdf_cli --quiet 2>/dev/null | grep -A10 "Query results"

echo
echo "💾 State is automatically saved to rdf_state.json"
echo "📌 You can bookmark queries and run them later"
echo "📜 Query history is preserved between sessions"

echo
echo "🎯 Interactive Mode:"
echo "Run: cargo run --bin rdf_cli"
echo "Try commands like:"
echo "  - predicates"
echo "  - query lmdfb"
echo "  - bookmark deps 'query dependsOn'"
echo "  - run deps"
echo "  - history"
