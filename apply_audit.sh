#!/bin/bash

echo "🔧 Applying Process Audit Transformation"
echo "========================================"

INPUT_FILE=${1:-"test_process.rs"}
OUTPUT_FILE=${2:-"test_process_audited.rs"}

echo "📁 Input: $INPUT_FILE"
echo "📁 Output: $OUTPUT_FILE"

# Create the audit macro and transform the file
cat > "$OUTPUT_FILE" << 'EOF'
// Auto-generated process audit wrapper
// All process executions are now audited

macro_rules! audit_execute {
    ($cmd:expr) => {{
        let start_time = std::time::Instant::now();
        println!("🔍 AUDIT: Executing command at {:?}", start_time);
        println!("📋 Command: {}", stringify!($cmd));
        
        let result = $cmd;
        
        let duration = start_time.elapsed();
        match &result {
            Ok(output) => {
                println!("✅ SUCCESS: Command completed in {:?}", duration);
                if let Some(status) = output.status.code() {
                    println!("📤 Exit code: {}", status);
                }
                if !output.stdout.is_empty() {
                    println!("📝 Stdout: {}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    println!("⚠️ Stderr: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                println!("❌ ERROR: Command failed in {:?}: {}", duration, e);
            }
        }
        
        result
    }};
}

EOF

# Transform the original file by replacing process calls
sed 's/Command::new(/audit_execute!(Command::new(/g' "$INPUT_FILE" | \
sed 's/\.output()/.output())/g' | \
sed 's/std::process::Command::new(/audit_execute!(std::process::Command::new(/g' >> "$OUTPUT_FILE"

echo "✅ Process audit transformation complete!"
echo "📋 Transformations applied:"
echo "   • Command::new() -> audit_execute!(Command::new())"
echo "   • Added timing and result capture"
echo "   • Added stdout/stderr logging"

echo
echo "🔍 Transformed file preview:"
head -20 "$OUTPUT_FILE"
