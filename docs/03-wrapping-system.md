# Split-Decls-RS Documentation Series

## Part 3: Wrapping System Deep Dive

### How Code Transformation Works

The wrapping system performs a sophisticated AST-level transformation that converts monolithic Rust crates into modular, addressable declaration spaces.

### The Wrapping Process

#### Phase 1: Backup & Preparation

```rust
// Original structure:
src/lib.rs          // Main library file
build.rs            // Optional build script

// After wrapping:
src/oldlib.rs       // Backup of original lib.rs
src/oldbuild.rs     // Backup of original build.rs (if exists)
src/lib.rs          // New gateway module
build.rs            // New orchestrating build script
```

#### Phase 2: AST Parsing & Declaration Extraction

The system uses `syn` to parse the original `lib.rs`:

```rust
// Example original code in lib.rs:
pub type Error = gimli::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugFile {
    Primary,
    Supplementary,
    Dwo,
}

pub struct Context<R> {
    // ... fields
}
```

Each declaration becomes a separate file:

```rust
// src/decls/wrapped_addr2line_decls_Error.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
type Error = gimli::Error;

// src/decls/wrapped_addr2line_decls_DebugFile.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DebugFile {
    Primary,
    Supplementary,
    Dwo,
}
```

#### Phase 3: Gateway Module Generation

The new `lib.rs` becomes a re-export gateway:

```rust
// Generated src/lib.rs
use introspector_decl2_macros::prelude;
pub mod decls;
pub use decls::*;
```

#### Phase 4: Build Script Orchestration

The generated `build.rs` handles the transformation:

```rust
// Key responsibilities:
// 1. Parse oldlib.rs into AST
// 2. Extract individual declarations
// 3. Generate declaration files with common imports
// 4. Create module invocation file
// 5. Set up cargo rerun triggers
```

### Declaration Types Supported

| Type | Example | Generated File |
|------|---------|----------------|
| **Type Alias** | `type Error = gimli::Error;` | `*_decls_Error.rs` |
| **Struct** | `struct Context<R> { ... }` | `*_decls_Context.rs` |
| **Enum** | `enum DebugFile { ... }` | `*_decls_DebugFile.rs` |
| **Function** | `pub fn demangle(...) { ... }` | `*_decls_demangle.rs` |
| **Trait** | `trait MyTrait { ... }` | `*_decls_MyTrait.rs` |
| **Impl Block** | `impl Context { ... }` | `*_decls_impl_for_Context.rs` |
| **Const** | `const MAX_SIZE: usize = 1024;` | `*_decls_MAX_SIZE.rs` |
| **Static** | `static GLOBAL: AtomicU32 = ...;` | `*_decls_GLOBAL.rs` |

### Common Use Statement Injection

Every generated declaration file includes:

```rust
// Automatically injected into every declaration file:
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Plus any use statements from the original file:
use gimli;
use std::fmt;
// ... other imports from oldlib.rs
```

### Module Not Found Handling

When the system encounters missing modules, it generates placeholder files:

```rust
// src/decls/wrapped_addr2line_decls_module_not_found_tests.rs
#[llm_error_message(message = "Module file not found for: tests")]
#[llm_context(crate_name = "wrapped_addr2line", module_name = "tests")]
pub struct wrapped_addr2line_decls_module_not_found_tests;
```

### Configuration System

Each wrapped crate gets a configuration file:

```toml
# .split-decls-config.toml
[crate_info]
name = "wrapped-addr2line"
original_name = "addr2line"
wrapped_at = "2024-12-26T16:09:00Z"

[wrapping_stats]
total_declarations = 18
functions = 2
structs = 4
enums = 3
type_aliases = 2
impl_blocks = 5
module_not_found = 2
```

### Advanced Features

#### Prelude Macro Integration

```rust
// Each declaration file can use prelude macros:
prelude! {
    // Custom prelude code injected here
}

#[decl_addr2line]  // Attribute for declaration metadata
type Error = gimli::Error;
```

#### Patch System Integration

The wrapper integrates with the patch system from `split-decls-rs.toml`:

```toml
[patches]
"addr2line_error_patch.rs" = ["wrapped-addr2line"]

[string_replacements]
"gimli::Error" = "CustomError"

[custom_prelude_overlay]
addr2line = """
use custom_error_handling::*;
"""
```

### Error Handling & Recovery

The system gracefully handles various edge cases:

- **Unsupported syntax**: Skips with warning, continues processing
- **Parse errors**: Falls back to string-based processing
- **Missing dependencies**: Generates placeholder modules
- **Circular references**: Detected and resolved through module system

### Performance Characteristics

| Metric | addr2line Example | Large Crate (1000+ decls) |
|--------|-------------------|---------------------------|
| **Parse Time** | ~50ms | ~2-5s |
| **File Generation** | ~100ms | ~10-30s |
| **Total Wrap Time** | ~200ms | ~1-2min |
| **Output Size** | 18 files, ~50KB | 1000+ files, ~10MB |

### Debugging Wrapped Code

```bash
# Check what was generated
ls output2/wrapped-addr2line/src/decls/ | head -5

# Examine a specific declaration
cat output2/wrapped-addr2line/src/decls/wrapped_addr2line_decls_Error.rs

# Verify build system works
cd output2/wrapped-addr2line && cargo check

# Test individual declarations
cargo run --bin test_addr2line_module
```

### Integration Points

The wrapping system integrates with:

1. **Cargo Workspace System** - Generates proper `Cargo.toml`
2. **Build System** - Creates orchestrating `build.rs`
3. **Macro System** - Enables `Output2MacroSystem` import
4. **Address Mapping** - Provides declarations for `decl2addr!`
5. **Evaluation Engine** - Supplies callable modules

### Next Steps

- **Part 4**: Learn how declarations map to memory addresses
- **Part 5**: Understand the macro system and evaluation engine
- **Part 6**: Explore advanced features and integrations

---
*The wrapping system transforms static Rust code into a dynamic, modular, and addressable declaration space.*
