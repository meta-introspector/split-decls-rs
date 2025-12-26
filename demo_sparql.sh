#!/bin/bash

echo "🔍 SPARQL-like Query Demo for LMDFB Knowledge Base"
echo "=================================================="
echo

echo "📊 1. Find declarations with MAXIMUM complexity:"
echo "maxcomplexity" | cargo run --bin rdf_cli --quiet 2>/dev/null | grep -A10 "Maximum Complexity"

echo
echo "🎯 2. Find declarations with complexity >= 12:"
echo "complexity 12" | cargo run --bin rdf_cli --quiet 2>/dev/null | grep -A15 "complexity >= 12"

echo
echo "📌 3. Bookmark and run complex queries:"
echo -e "bookmark highcomplex 'complexity 11'\nrun highcomplex\nquit" | cargo run --bin rdf_cli --quiet 2>/dev/null | grep -A10 "complexity >= 11"

echo
echo "💡 Available SPARQL-like commands:"
echo "  - maxcomplexity                    # Find max complexity declarations"
echo "  - complexity <threshold>           # Find declarations above threshold"
echo "  - query <predicate>               # Find triples with predicate"
echo "  - bookmark <name> <query>         # Save query for reuse"
echo "  - run <bookmark>                  # Execute saved query"
echo "  - history                         # Show query history"
echo
echo "🚀 Start interactive mode: cargo run --bin rdf_cli"
