#!/bin/bash

# Long-running auto-fix test with proper output capture
TARGET="${1:-rustc_driver_impl::lib::main}"
OUTPUT_FILE="autofix_results_$(date +%Y%m%d_%H%M%S).log"

echo "🚀 Starting long-running auto-fix test for: $TARGET"
echo "📝 Output will be saved to: $OUTPUT_FILE"
echo "⏰ Started at: $(date)"

# Build first
echo "🔨 Building unified_driver..."
cargo build --bin unified_driver --quiet

# Run with nice priority and capture output
nice -n 10 timeout 30m ./target/debug/unified_driver "$TARGET" > "$OUTPUT_FILE" 2>&1 &
PID=$!

echo "🔄 Process ID: $PID"
echo "📊 Monitoring progress (Ctrl+C to stop monitoring, process continues)..."

# Monitor progress
while kill -0 $PID 2>/dev/null; do
    # Show last few lines and stats
    echo "⏰ $(date) - Process still running"
    echo "📈 Auto-fixes so far: $(grep -c "🔧 AUTO-FIX: Found" "$OUTPUT_FILE" 2>/dev/null || echo 0)"
    echo "⚠️  Skipped so far: $(grep -c "⚠️  SKIPPING" "$OUTPUT_FILE" 2>/dev/null || echo 0)"
    echo "📊 File size: $(du -h "$OUTPUT_FILE" 2>/dev/null | cut -f1 || echo "0B")"
    echo "---"
    
    sleep 30
done

# Wait for process to complete
wait $PID
EXIT_CODE=$?

echo "✅ Process completed at: $(date)"
echo "🔚 Exit code: $EXIT_CODE"
echo "📊 Final stats:"
echo "   Auto-fixes: $(grep -c "🔧 AUTO-FIX: Found" "$OUTPUT_FILE" 2>/dev/null || echo 0)"
echo "   Skipped: $(grep -c "⚠️  SKIPPING" "$OUTPUT_FILE" 2>/dev/null || echo 0)"
echo "   File size: $(du -h "$OUTPUT_FILE" 2>/dev/null | cut -f1 || echo "0B")"

# Show final result
if grep -q "✅ SUCCESS" "$OUTPUT_FILE" 2>/dev/null; then
    echo "🎉 COMPILATION SUCCESSFUL!"
    grep "✅ SUCCESS" "$OUTPUT_FILE" | tail -1
else
    echo "❌ Process ended without success"
    echo "📋 Last 10 lines:"
    tail -10 "$OUTPUT_FILE" 2>/dev/null || echo "No output file"
fi

echo "📁 Full output saved in: $OUTPUT_FILE"
