#!/bin/bash

echo "🎯 Starting complete rustc analysis with full backtrace..."
echo "📅 Started at: $(date)"

# Set environment variables
export RUST_BACKTRACE=full
export RUST_LOG=debug

# Create output files
OUTPUT_FILE="rustc_analysis_$(date +%Y%m%d_%H%M%S).txt"
LOG_FILE="rustc_analysis_$(date +%Y%m%d_%H%M%S).log"

echo "📁 Output file: $OUTPUT_FILE"
echo "📁 Log file: $LOG_FILE"

# Run the analysis with timeout and capture everything
timeout 300 cargo run --bin complete_rustc_analysis > "$OUTPUT_FILE" 2> "$LOG_FILE"
EXIT_CODE=$?

echo "📊 Analysis completed with exit code: $EXIT_CODE"
echo "📅 Finished at: $(date)"

# Show file sizes
echo "📄 Output file size: $(wc -l < "$OUTPUT_FILE") lines, $(du -h "$OUTPUT_FILE" | cut -f1)"
echo "📄 Log file size: $(wc -l < "$LOG_FILE") lines, $(du -h "$LOG_FILE" | cut -f1)"

# Show last few lines of each file
echo ""
echo "📋 Last 10 lines of output:"
tail -10 "$OUTPUT_FILE"

echo ""
echo "📋 Last 10 lines of log:"
tail -10 "$LOG_FILE"

# Check if analysis completed
if grep -q "📊 Complete Function Call Analysis" "$OUTPUT_FILE"; then
    echo "✅ Analysis completed successfully!"
    grep -A 5 "📊 Complete Function Call Analysis" "$OUTPUT_FILE"
else
    echo "⚠️  Analysis may have been interrupted or timed out"
fi

echo ""
echo "🔍 Top function calls:"
sort "$OUTPUT_FILE" | uniq -c | sort -rn | head -5
