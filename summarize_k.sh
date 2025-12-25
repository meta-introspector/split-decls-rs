#!/bin/bash
# K-theory node summarizer script

if [ $# -eq 0 ]; then
    echo "Usage: $0 <k-address>"
    echo "Examples: k7.1, k7.8, k7.-1"
    exit 1
fi

ADDR="$1"
INDEX_FILE="k_theory_index.json"

if [ ! -f "$INDEX_FILE" ]; then
    echo "❌ K-theory index not found. Run 'make indexer' first."
    exit 1
fi

# Handle k7.-1 (last node) special case
if [[ "$ADDR" == *".-1" ]]; then
    LEVEL=$(echo "$ADDR" | sed 's/k\([0-9]\+\)\.-1/\1/')
    echo "🔍 K$LEVEL.-1 (last node):"
    jq -r ".nodes | to_entries | map(select(.key | startswith(\"k$LEVEL.\"))) | last | .value | \"📍 \(.name) (complexity: \(.complexity), depth: \(.depth))\\n📄 \(.content)\"" "$INDEX_FILE"
else
    echo "🔍 $ADDR:"
    jq -r ".nodes.\"$ADDR\" | if . then \"📍 \(.name) (complexity: \(.complexity), depth: \(.depth))\\n📄 \(.content)\" else \"❌ Node not found\" end" "$INDEX_FILE"
fi
