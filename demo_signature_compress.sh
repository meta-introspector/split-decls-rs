#!/bin/bash

echo "🔢 Signature Compression Demo: Prime Numbers & Emojis"
echo "===================================================="

echo ""
echo "📋 Creating test declaration files..."

# Create test directory with various declaration types
mkdir -p test_decls

# Function declaration
cat > test_decls/func_decl.rs << 'EOF'
prelude!{}
#[decl_test]
use std::collections::HashMap;
pub fn process_data(input: &str) -> Result<String, Error> {
    Ok(input.to_uppercase())
}
EOF

# Struct declaration  
cat > test_decls/struct_decl.rs << 'EOF'
prelude!{}
#[decl_test]
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProcessor {
    pub name: String,
    pub version: u32,
}
EOF

# Enum declaration
cat > test_decls/enum_decl.rs << 'EOF'
prelude!{}
#[decl_test]
use std::fmt;
#[derive(Debug, Clone)]
pub enum ProcessingState {
    Idle,
    Processing(String),
    Complete { result: String, duration: u64 },
}
EOF

# Trait declaration
cat > test_decls/trait_decl.rs << 'EOF'
prelude!{}
#[decl_test]
use async_trait::async_trait;
#[async_trait]
pub trait DataProcessor {
    async fn process(&self, data: &str) -> Result<String, Error>;
    fn name(&self) -> &str;
}
EOF

# Impl block
cat > test_decls/impl_decl.rs << 'EOF'
prelude!{}
#[decl_test]
use std::collections::HashMap;
impl DataProcessor for MyProcessor {
    async fn process(&self, data: &str) -> Result<String, Error> {
        Ok(data.to_string())
    }
    fn name(&self) -> &str { "MyProcessor" }
}
EOF

# Macro declaration
cat > test_decls/macro_decl.rs << 'EOF'
prelude!{}
#[decl_test]
macro_rules! generate_processor {
    ($name:ident) => {
        pub struct $name;
        impl DataProcessor for $name {
            fn process(&self, data: &str) -> String {
                format!("Processed by {}: {}", stringify!($name), data)
            }
        }
    };
}
EOF

# Complex declaration (multiple bindings)
cat > test_decls/complex_decl.rs << 'EOF'
prelude!{}
#[decl_test]
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexProcessor {
    data: HashMap<String, String>,
}
#[async_trait]
impl DataProcessor for ComplexProcessor {
    async fn process(&self, input: &str) -> Result<String, Error> {
        Ok(format!("Complex: {}", input))
    }
}
EOF

# Simple declaration (minimal bindings)
cat > test_decls/simple_decl.rs << 'EOF'
prelude!{}
pub fn simple() {}
EOF

echo "✅ Created 8 test declaration files"

echo ""
echo "🔨 Building signature compression tool..."
cargo build --bin signature_compress

echo ""
echo "🔢 Running signature compression analysis..."
cargo run --bin signature_compress -- -i test_decls -o signature_mappings.txt -s --optimize

echo ""
echo "📊 Results:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -f signature_mappings.txt ]; then
    echo "📋 Signature mappings:"
    cat signature_mappings.txt
fi

echo ""
if [ -f test_decls_lookup.rs ]; then
    echo "🔍 Generated lookup code (first 20 lines):"
    head -20 test_decls_lookup.rs
fi

echo ""
echo "🎯 Key Insights:"
echo "  ✓ Most common signature gets prime 2 (50% probability)"
echo "  ✓ Each unique binding pattern gets a unique prime"
echo "  ✓ Emojis provide visual representation of signature types"
echo "  ✓ Frequency-based optimization minimizes average prime size"
echo "  ✓ Lookup tables enable fast signature decoding"

echo ""
echo "🧮 Mathematical Properties:"
echo "  • Prime 2: Most common pattern (e.g., simple functions)"
echo "  • Prime 3: Second most common (e.g., basic structs)"  
echo "  • Prime 5, 7, 11...: Increasingly rare patterns"
echo "  • Compression ratio = frequency of most common / total"
echo "  • Each declaration can be identified by its prime signature"

echo ""
echo "🎨 Emoji Key Examples:"
echo "  🔧 Functions (pub_fn)"
echo "  🏗️ Structs (pub_struct)" 
echo "  🎯 Enums (pub_enum)"
echo "  ⚙️ Implementations (impl_block)"
echo "  🎭 Traits (trait_def)"
echo "  🪄 Macros (macro_def)"
echo "  ✨ Derives (derive_attr)"
echo "  🌟 Preludes (prelude)"

echo ""
echo "✅ Demo complete! Check signature_mappings.txt and test_decls_lookup.rs"
