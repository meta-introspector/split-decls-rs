# 🔮 Parameterized Layer Macros

## Concept
Each layer in the rustc crystal takes **N arguments** representing all lower layers as dependencies.

## Layer Signature Pattern

```rust
// Layer 0: No dependencies (foundation)
macro_rules! rustc_layer_0 {
    () => { /* 2 functions */ };
}

// Layer 1: Takes layer 0 as dependency  
macro_rules! rustc_layer_1 {
    ($layer_0:expr) => { /* 15 functions */ };
}

// Layer 2: Takes layers 0,1 as dependencies
macro_rules! rustc_layer_2 {
    ($layer_0:expr, $layer_1:expr) => { /* 42 functions */ };
}

// Layer N: Takes layers 0..N-1 as dependencies
macro_rules! rustc_layer_N {
    ($layer_0:expr, $layer_1:expr, ..., $layer_{N-1}:expr) => {
        /* Layer N functions with access to all lower layers */
    };
}
```

## Composition Chain

```rust
macro_rules! compose_rustc_crystal {
    () => {
        let layer_0 = rustc_layer_0!();
        let layer_1 = rustc_layer_1!(layer_0);
        let layer_2 = rustc_layer_2!(layer_0, layer_1);
        let layer_3 = rustc_layer_3!(layer_0, layer_1, layer_2);
        // ...
        let layer_41 = rustc_layer_41!(layer_0, layer_1, ..., layer_40);
        layer_41 // Return complete rustc
    };
}
```

## Key Properties

### 🔗 **Dependency Injection**
Each layer receives **all lower layers** as parameters:
- Layer N can call any function from layers 0..N-1
- Guarantees dependency satisfaction
- Enables modular composition

### 📊 **Argument Growth**
- Layer 0: 0 arguments
- Layer 1: 1 argument (layer_0)
- Layer 2: 2 arguments (layer_0, layer_1)
- Layer N: N arguments (layer_0, ..., layer_{N-1})
- Layer 41: 41 arguments (all lower layers)

### 🏗️ **Build Strategy**
```rust
// Manual layer-by-layer construction
let foundation = rustc_layer_0!();
let core = rustc_layer_1!(foundation);
let build = rustc_layer_2!(foundation, core);
let logic = rustc_layer_3!(foundation, core, build);
// ... continue building up

// Or automatic composition
let complete_rustc = compose_rustc_crystal!();
```

## Benefits

1. **🔒 Type Safety**: Compile-time dependency verification
2. **🔄 Composability**: Mix and match layer implementations  
3. **🎯 Modularity**: Each layer is independently testable
4. **📈 Scalability**: Add new layers without breaking existing ones
5. **🔍 Traceability**: Clear dependency chain from foundation to apex

## Usage Example

```rust
// Custom layer implementations
rustc_layer_0! { /* foundation implementation */ }
rustc_layer_1! { |foundation| /* core using foundation */ }
rustc_layer_2! { |foundation, core| /* build using both */ }

// Compose into complete compiler
let rustc = compose_rustc_crystal!();
```

This creates a **parameterized crystal lattice** where each layer is a function of all its dependencies!
