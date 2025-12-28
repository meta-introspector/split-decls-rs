macro_rules! custom_rust_driver_impl {
    () => {
        # [decl (fn , name = "custom_rust_driver_impl" , vis = "pub" , hash = "f622bd41")] pub fn custom_rust_driver_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let config = input_str . value () ; quote ! { { println ! ("cargo:warning=🚀 Creating custom rust driver") ; let driver_code = format ! (r#"
#!/bin/bash
# Custom Rust Driver - Automated rustc source analysis
# Config: {}

set -e

echo "🚀 Custom Rust Driver Starting..."

# 1. Detect rustc
RUSTC_PATH=$(which rustc)
REAL_RUSTC=$(readlink -f "$RUSTC_PATH")
echo "📍 Rustc found: $REAL_RUSTC"

# 2. Get version and commit
RUSTC_VERSION=$(rustc --version --verbose)
COMMIT_HASH=$(echo "$RUSTC_VERSION" | grep "commit-hash:" | cut -d: -f2 | tr -d ' ')
echo "🔖 Commit: $COMMIT_HASH"

# 3. Determine source URL
if [ -n "$COMMIT_HASH" ] && [ "$COMMIT_HASH" != "unknown" ]; then
    SOURCE_URL="https://github.com/rust-lang/rust/archive/$COMMIT_HASH.tar.gz"
else
    SOURCE_URL="https://github.com/rust-lang/rust/archive/master.tar.gz"
fi
echo "🌐 Source URL: $SOURCE_URL"

# 4. Create workspace
WORKSPACE="./rust-analysis-workspace"
mkdir -p "$WORKSPACE"
cd "$WORKSPACE"

# 5. Download and extract source
if [ ! -d "rust-src" ]; then
    echo "📥 Downloading rust source..."
    curl -L "$SOURCE_URL" | tar xz
    mv rust-* rust-src
    echo "✅ Source extracted to: $(pwd)/rust-src"
fi

# 6. Analyze source structure
echo "🔍 Analyzing source structure..."
cd rust-src

# Count files by type
echo "📊 File analysis:"
find . -name "*.rs" | wc -l | xargs echo "  Rust files:"
find . -name "*.toml" | wc -l | xargs echo "  TOML files:"
find . -name "*.md" | wc -l | xargs echo "  Markdown files:"

# Analyze compiler structure
echo "🧮 Compiler structure:"
ls -la compiler/ 2>/dev/null || echo "  No compiler/ directory"
ls -la src/librustc* 2>/dev/null || echo "  No librustc* directories"
ls -la src/rustc* 2>/dev/null || echo "  No rustc* directories"

# Count keywords in source
echo "🔍 Keyword analysis:"
echo "  fn declarations: $(find . -name "*.rs" -exec grep -c "^fn " {{}} \; | awk '{{sum += $1}} END {{print sum}}')"
echo "  struct definitions: $(find . -name "*.rs" -exec grep -c "^struct " {{}} \; | awk '{{sum += $1}} END {{print sum}}')"
echo "  impl blocks: $(find . -name "*.rs" -exec grep -c "^impl " {{}} \; | awk '{{sum += $1}} END {{print sum}}')"
echo "  trait definitions: $(find . -name "*.rs" -exec grep -c "^trait " {{}} \; | awk '{{sum += $1}} END {{print sum}}')"

# 7. Generate analysis report
cat > ../analysis_report.json << EOF
{{
  "rustc_binary": "$REAL_RUSTC",
  "source_url": "$SOURCE_URL", 
  "commit_hash": "$COMMIT_HASH",
  "source_path": "$(pwd)",
  "analysis_timestamp": "$(date -Iseconds)",
  "workspace": "$WORKSPACE"
}}
EOF

echo "✅ Analysis complete!"
echo "📄 Report saved to: $WORKSPACE/analysis_report.json"
echo "📁 Source available at: $(pwd)"

# 8. Build custom rustc (optional)
if [ "$1" = "--build" ]; then
    echo "🔨 Building custom rustc..."
    ./configure --enable-debug
    make -j$(nproc)
    echo "✅ Custom rustc built!"
fi
                "# , # config) ; driver_code } } . into () }
    };
}

custom_rust_driver_impl!()