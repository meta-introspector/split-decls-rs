#!/bin/bash

echo "🔧 Applying Full Syscall Audit to Bootstrap"
echo "==========================================="

# Create comprehensive audit macro for both process and file operations
cat > bootstrap_audit_macros.rs << 'EOF'
// Comprehensive syscall audit macros for bootstrap
use std::time::{SystemTime, Instant};

macro_rules! audit_execute {
    ($cmd:expr) => {{
        let start = Instant::now();
        let timestamp = SystemTime::now();
        println!("⚠️  PROCESS AUDIT: {:?}", timestamp);
        println!("📋 Command: {}", stringify!($cmd));
        println!("📁 PWD: {:?}", std::env::current_dir().unwrap_or_default());
        
        let result = $cmd;
        let duration = start.elapsed();
        
        match &result {
            Ok(output) => {
                println!("✅ Process completed in {:?}", duration);
                if let Some(code) = output.status.code() {
                    println!("📤 Exit: {}", code);
                }
            }
            Err(e) => println!("❌ Process failed: {}", e),
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        result
    }};
}

macro_rules! audit_fs_write {
    ($path:expr, $contents:expr) => {{
        let timestamp = SystemTime::now();
        println!("⚠️  FILE WRITE AUDIT: {:?}", timestamp);
        println!("📝 Writing to: {:?}", $path);
        println!("📊 Size: {} bytes", $contents.len());
        
        let result = std::fs::write($path, $contents);
        
        match &result {
            Ok(_) => println!("✅ File written successfully"),
            Err(e) => println!("❌ File write failed: {}", e),
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        result
    }};
}

macro_rules! audit_fs_create_dir_all {
    ($path:expr) => {{
        let timestamp = SystemTime::now();
        println!("⚠️  DIR CREATE AUDIT: {:?}", timestamp);
        println!("📁 Creating: {:?}", $path);
        
        let result = std::fs::create_dir_all($path);
        
        match &result {
            Ok(_) => println!("✅ Directory created successfully"),
            Err(e) => println!("❌ Directory creation failed: {}", e),
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        result
    }};
}
EOF

echo "✅ Created comprehensive audit macros"

# Create transformation script for Rust source files
cat > transform_bootstrap_source.sh << 'EOF'
#!/bin/bash

FILE="$1"
OUTPUT="$2"

if [ ! -f "$FILE" ]; then
    echo "File not found: $FILE"
    exit 1
fi

echo "🔧 Transforming $FILE -> $OUTPUT"

# Add audit macros at the top
cat bootstrap_audit_macros.rs > "$OUTPUT"
echo "" >> "$OUTPUT"

# Transform the source file
sed 's/std::process::Command/audit_execute!(std::process::Command/g' "$FILE" | \
sed 's/Command::new(/audit_execute!(Command::new(/g' | \
sed 's/\.output()/.output())/g' | \
sed 's/\.status()/.status())/g' | \
sed 's/std::fs::write(/audit_fs_write!(/g' | \
sed 's/fs::write(/audit_fs_write!(/g' | \
sed 's/std::fs::create_dir_all(/audit_fs_create_dir_all!(/g' | \
sed 's/fs::create_dir_all(/audit_fs_create_dir_all!(/g' >> "$OUTPUT"

echo "✅ Transformation complete"
EOF

chmod +x transform_bootstrap_source.sh

echo "✅ Created source transformation script"

# Find and transform bootstrap-related source files
echo "🔍 Finding bootstrap source files..."

BOOTSTRAP_FILES=$(find src/bin -name "*bootstrap*" -o -name "split-decls-rs.rs" -o -name "main.rs" | head -5)

echo "📋 Bootstrap files found:"
echo "$BOOTSTRAP_FILES"

# Transform each file
for file in $BOOTSTRAP_FILES; do
    if [ -f "$file" ]; then
        output_file="${file%.rs}_audited.rs"
        echo "🔧 Transforming: $file -> $output_file"
        ./transform_bootstrap_source.sh "$file" "$output_file"
    fi
done

# Update Cargo.toml to use audited binaries
echo "📝 Creating audited binary entries..."

cat >> Cargo.toml << 'EOF'

# Audited bootstrap binaries
[[bin]]
name = "bootstrap_audited"
path = "src/bin/bootstrap-self-apply_audited.rs"

[[bin]]
name = "split_decls_rs_audited"
path = "src/bin/split-decls-rs_audited.rs"
EOF

echo "✅ Added audited binaries to Cargo.toml"

echo
echo "🎯 Usage:"
echo "   cargo run --bin bootstrap_audited"
echo "   cargo run --bin split_decls_rs_audited -- bootstrap"

echo
echo "📊 Audit Output Will Show:"
echo "   ⚠️  PROCESS AUDIT: [timestamp]"
echo "   📋 Command: [full command]"
echo "   📁 PWD: [working directory]"
echo "   ✅ Process completed in [duration]"
echo "   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "   ⚠️  FILE WRITE AUDIT: [timestamp]"
echo "   📝 Writing to: [file path]"
echo "   📊 Size: [bytes]"
echo "   ✅ File written successfully"
echo "   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
