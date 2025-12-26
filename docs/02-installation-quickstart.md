# Split-Decls-RS Documentation Series

## Part 2: Installation & Quick Start

### Prerequisites

- Rust 1.70+ with Cargo
- Linux/Unix environment (tested on Linux)
- `sccache` for faster builds (optional but recommended)
- Standard Unix tools: `nm`, `objdump`, `readelf`

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd split-decls-rs

# Build the system
cargo build --release

# Verify installation
cargo run --bin split-decls-rs -- --help
```

### Quick Start: Wrapping Your First Crate

#### Step 1: Wrap a Single Crate

```bash
# Wrap the addr2line crate (example)
make wrap_addr2line

# Or wrap any crate manually
cargo run --bin wrap_single_crate -- ../your-crate --verbose
```

**What happens:**
- Original `lib.rs` → `oldlib.rs` (backup)
- Declarations split into `src/decls/` directory
- New `lib.rs` and `build.rs` generated
- 18+ individual declaration files created

#### Step 2: Test the Wrapped Code

```bash
# Test the wrapped functionality
make test_addr2line_module

# See the output:
# ✅ Error type - created and used successfully
# ✅ DebugFile enum - all variants created and compared
# ✅ RangeAttributes struct - created with default and custom
# ✅ Pattern matching - Primary variant matched
# ✅ Debug formatting - DebugFile formats correctly
# ✅ Cloning - DebugFile clones correctly
```

#### Step 3: Map Declarations to Addresses

```bash
# Generate address mappings for all declarations
make proof_decl2addr

# Output shows:
# 📍 Individual declaration mapping (REAL ADDRESSES):
#   Error → 0x5e5f6da0
#   DebugFile → 0x879548f3
#   Context → 0x92ce1f4c
# 📊 Total declarations mapped: 3328
```

#### Step 4: Analyze Common Terms

```bash
# Find common patterns in declaration names
make analyze_terms

# Results:
# 🏆 Most common terms:
#   error appears in 47 declarations
#   file appears in 38 declarations
#   kind appears in 34 declarations
```

### Make Targets Reference

| Target | Description |
|--------|-------------|
| `make wrap_addr2line` | Wrap the addr2line crate specifically |
| `make test_addr2line` | Test lisp-like macro system |
| `make test_addr2line_module` | Exercise wrapped code directly |
| `make proof_wrap_bin` | Prove !wrap_bin macro functionality |
| `make proof_decl2addr` | Demonstrate address mapping system |
| `make analyze_terms` | Analyze common terms and real addresses |
| `make gen_workspace` | Generate workspace with all dependencies |
| `make gen_workspace_quiet` | Generate workspace silently |

### Directory Structure After Wrapping

```
output2/
└── wrapped-addr2line/
    ├── Cargo.toml                    # Generated workspace config
    ├── build.rs                      # Generated build script
    ├── .split-decls-config.toml      # Wrapper configuration
    └── src/
        ├── lib.rs                    # New gateway module
        ├── oldlib.rs                 # Original lib.rs backup
        └── decls/                    # Split declarations
            ├── wrapped_addr2line_decls_Error.rs
            ├── wrapped_addr2line_decls_DebugFile.rs
            ├── wrapped_addr2line_decls_Context.rs
            └── ... (15+ more files)
```

### Verification Commands

```bash
# Check if wrapping succeeded
ls output2/wrapped-addr2line/src/decls/ | wc -l
# Should show 18+ files

# Verify address mapping works
cargo run --bin analyze_common_terms | grep "Found real addresses"
# Should show: ✅ Found real addresses: 50

# Test macro system
cargo run --bin test_addr2line | grep "macro declarations"
# Should show: 📦 Imported 3328+ macro declarations from output2
```

### Troubleshooting

**Build Errors:**
- Ensure Rust 1.70+: `rustc --version`
- Clear cache: `cargo clean`
- Check dependencies: `cargo check`

**Wrapping Failures:**
- Verify target crate exists: `ls ../addr2line`
- Check permissions: `ls -la output2/`
- Run with verbose: `--verbose` flag

**Address Resolution Issues:**
- Install debug tools: `sudo apt install binutils`
- Check binary symbols: `nm /proc/self/exe | head`

### Next Steps

- **Part 3**: Learn how the wrapping system transforms code
- **Part 4**: Understand address mapping and memory layout
- **Part 5**: Master the macro system and evaluation engine

---
*You now have a working split-decls-rs installation and have successfully wrapped your first crate!*
