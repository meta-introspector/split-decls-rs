#!/bin/bash

echo "🔧 Applying Process Audit to Bootstrap and Output3 Commands"
echo "=========================================================="

# Create audited Makefile
cp Makefile Makefile.original
cp Makefile Makefile.audited

echo "📋 Found cargo run commands to audit:"
grep -n "cargo run" Makefile | head -10

echo
echo "🛡️ Creating audit wrapper script..."

cat > cargo_audit_wrapper.sh << 'EOF'
#!/bin/bash

# Audit wrapper for cargo run commands
echo "🔍 AUDIT: Executing cargo command at $(date)"
echo "📋 Command: cargo $*"
echo "📁 Working directory: $(pwd)"

start_time=$(date +%s.%N)

# Execute the actual cargo command
cargo "$@"
exit_code=$?

end_time=$(date +%s.%N)
duration=$(echo "$end_time - $start_time" | bc -l)

if [ $exit_code -eq 0 ]; then
    echo "✅ SUCCESS: Cargo command completed in ${duration}s"
    echo "📤 Exit code: $exit_code"
else
    echo "❌ ERROR: Cargo command failed in ${duration}s"
    echo "📤 Exit code: $exit_code"
fi

echo "----------------------------------------"
exit $exit_code
EOF

chmod +x cargo_audit_wrapper.sh

echo "✅ Created cargo audit wrapper"

echo
echo "🔧 Modifying Makefile to use audit wrapper..."

# Replace cargo run with audited version
sed -i 's/cargo run/\.\/cargo_audit_wrapper.sh run/g' Makefile.audited

echo "✅ Modified Makefile.audited"

echo
echo "📊 Audit transformations applied:"
echo "   • run_bootstrap: Now audited"
echo "   • generate_output3_from_enhanced: Now audited"
echo "   • All cargo run commands: Now audited"

echo
echo "🎯 Usage:"
echo "   make -f Makefile.audited run_bootstrap"
echo "   make -f Makefile.audited enhanced_generation"

echo
echo "📋 Audit output will show:"
echo "   🔍 AUDIT: Executing cargo command at [timestamp]"
echo "   📋 Command: cargo run --bin [binary] [args]"
echo "   📁 Working directory: [path]"
echo "   ✅ SUCCESS: Cargo command completed in [duration]s"
echo "   📤 Exit code: [code]"
