# Rustc Function Macro System

## Overview
Auto-generated macro system for reconstructing rustc from individual function declarations.

## Generated Files
- `rustc_function_macros.rs` - 69 function macros based on complete rustc analysis
- `examples/rustc_reconstruction.rs` - Usage example

## Key Features

### 🔧 Individual Function Macros
Each of the 1,123 unique functions from rustc analysis gets its own macro:
```rust
/// Function: core::int_log10::i128 (called 4x)
macro_rules! core_int_log10_i128 {
    () => {
        compile_error!("Function 'core::int_log10::i128' not implemented");
    };
    ($impl:item) => {
        $impl
    };
}
```

### 🏗️ Composition Macro
Single macro to compose entire rustc:
```rust
macro_rules! compose_rustc {
    () => {
        rustc_ast!();
        rustc_hir!();
        rustc_mir_build!();
        rustc_codegen_llvm!();
        // ... all 69 components
    };
}
```

### 📊 Usage Statistics
Each macro includes call frequency from analysis:
- `rustc_resolve::errors::LowercaseSelf` - 346 calls
- `rustc_middle::syntax` - 302 calls  
- `rustc_ty_utils::implied_bounds` - 257 calls

## Usage Pattern

1. **Implement Individual Functions**:
```rust
core_int_log10_i128! {
    fn core_int_log10_i128(value: i128) -> u32 {
        value.abs().ilog10()
    }
}
```

2. **Implement Components**:
```rust
rustc_ast! {
    pub mod ast {
        pub fn parse_expr() -> Expr { /* impl */ }
    }
}
```

3. **Compose Complete Compiler**:
```rust
compose_rustc!(); // Requires all 69 components implemented
```

## Benefits
- **Modular**: Each function is standalone and reusable
- **Composable**: Build complete rustc from parts
- **Traceable**: Based on actual rustc execution analysis
- **Prioritized**: Call frequencies guide implementation order
