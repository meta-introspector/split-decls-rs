#!/bin/bash

echo "📊 REAL Syscall Analysis Report"
echo "==============================="
echo

echo "🔍 Scanning codebase..."
TOTAL_FILES=$(find . -name "*.rs" | wc -l)
echo "📁 Total Rust files: $TOTAL_FILES"
echo

echo "📋 Syscall Usage Analysis (Real Data):"

FS_COUNT=$(grep -r "std::fs::" . --include="*.rs" | wc -l)
PROCESS_COUNT=$(grep -r "std::process::" . --include="*.rs" | wc -l)
ENV_COUNT=$(grep -r "std::env::" . --include="*.rs" | wc -l)
IO_COUNT=$(grep -r "std::io::" . --include="*.rs" | wc -l)
NET_COUNT=$(grep -r "std::net::" . --include="*.rs" | wc -l)
LIBC_COUNT=$(grep -r "libc::" . --include="*.rs" | wc -l)
TIME_COUNT=$(grep -r "std::time::" . --include="*.rs" | wc -l)
THREAD_COUNT=$(grep -r "std::thread::" . --include="*.rs" | wc -l)
SYNC_COUNT=$(grep -r "std::sync::" . --include="*.rs" | wc -l)

TOTAL_SYSCALLS=$((FS_COUNT + PROCESS_COUNT + ENV_COUNT + IO_COUNT + NET_COUNT + LIBC_COUNT + TIME_COUNT + THREAD_COUNT + SYNC_COUNT))

echo "   📁 Filesystem (std::fs::):     $FS_COUNT calls"
echo "   ⚡ Process (std::process::):    $PROCESS_COUNT calls"
echo "   🌍 Environment (std::env::):   $ENV_COUNT calls"
echo "   💾 IO (std::io::):             $IO_COUNT calls"
echo "   🌐 Network (std::net::):       $NET_COUNT calls"
echo "   🔧 Libc (libc::):              $LIBC_COUNT calls"
echo "   ⏰ Time (std::time::):         $TIME_COUNT calls"
echo "   🧵 Thread (std::thread::):     $THREAD_COUNT calls"
echo "   🔒 Sync (std::sync::):         $SYNC_COUNT calls"
echo
echo "📈 Total Syscalls Found: $TOTAL_SYSCALLS"

echo
echo "📊 Usage Distribution:"
if [ $TOTAL_SYSCALLS -gt 0 ]; then
    FS_PCT=$(echo "scale=1; $FS_COUNT * 100 / $TOTAL_SYSCALLS" | bc -l)
    PROCESS_PCT=$(echo "scale=1; $PROCESS_COUNT * 100 / $TOTAL_SYSCALLS" | bc -l)
    ENV_PCT=$(echo "scale=1; $ENV_COUNT * 100 / $TOTAL_SYSCALLS" | bc -l)
    IO_PCT=$(echo "scale=1; $IO_COUNT * 100 / $TOTAL_SYSCALLS" | bc -l)
    NET_PCT=$(echo "scale=1; $NET_COUNT * 100 / $TOTAL_SYSCALLS" | bc -l)
    
    echo "   📁 Filesystem: ${FS_PCT}%"
    echo "   ⚡ Process: ${PROCESS_PCT}%"
    echo "   🌍 Environment: ${ENV_PCT}%"
    echo "   💾 IO: ${IO_PCT}%"
    echo "   🌐 Network: ${NET_PCT}%"
fi

echo
echo "🎯 Top Syscall Files (Filesystem):"
grep -r "std::fs::" . --include="*.rs" -l | head -5 | while read file; do
    count=$(grep -c "std::fs::" "$file")
    echo "   • $file ($count uses)"
done

echo
echo "⚡ Top Syscall Files (Process):"
grep -r "std::process::" . --include="*.rs" -l | head -5 | while read file; do
    count=$(grep -c "std::process::" "$file")
    echo "   • $file ($count uses)"
done

echo
echo "🛡️ Security Risk Assessment (Real Data):"
if [ $PROCESS_COUNT -gt 0 ]; then
    echo "   🔴 Critical: Process calls found ($PROCESS_COUNT) - Requires DAO governance"
fi
if [ $NET_COUNT -gt 0 ]; then
    echo "   🔴 Critical: Network calls found ($NET_COUNT) - Requires validation"
fi
if [ $FS_COUNT -gt 50 ]; then
    echo "   🟡 Medium: High filesystem usage ($FS_COUNT) - Needs path validation"
fi
if [ $ENV_COUNT -gt 1000 ]; then
    echo "   🟢 Low: Environment calls ($ENV_COUNT) - High usage, low risk"
fi

echo
echo "🔧 Trait Decoupling Recommendations:"
echo "   1. Environment ($ENV_COUNT calls) - Generic bounds for performance"
echo "   2. IO ($IO_COUNT calls) - Dependency injection for testing"
echo "   3. Filesystem ($FS_COUNT calls) - Oracle validation for security"
echo "   4. Process ($PROCESS_COUNT calls) - DAO governance for safety"

echo
echo "✅ Real syscall analysis complete!"
echo "   Data source: $TOTAL_FILES Rust files in current directory"
echo "   Total syscalls identified: $TOTAL_SYSCALLS"
