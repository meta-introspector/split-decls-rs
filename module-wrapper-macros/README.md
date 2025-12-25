# Module Wrapper Testing Strategy

## Overview
Test individual wrapped modules before full bootstrap integration.

## Usage Examples

### 1. Simple Module Replacement
```rust
use module_wrapper_macros::use_wrapper;

// Replace std::alloc with our generated wrapper
use_wrapper!(std::alloc => crate::generated::alloc_wrapper);
```

### 2. Conditional Replacement with Feature Flags
```rust
use module_wrapper_macros::conditional_wrapper;

// Only use wrapper when feature is enabled
conditional_wrapper!(
    feature = "use_wrapped_hashbrown",
    original = hashbrown::HashMap,
    wrapper = crate::generated::hashbrown_wrapper::HashMap
);
```

### 3. Individual Module Testing
```rust
use module_wrapper_macros::test_wrapper;

// Generate tests for a specific wrapper
test_wrapper!(crate::generated::nom_wrapper, nom_tests);
```

## Testing Workflow

1. **Generate wrapper** for single crate (e.g., `nom`)
2. **Create test crate** that uses the wrapper
3. **Run isolated tests** to verify wrapper works
4. **Gradually integrate** into larger system

## Example Test Crate Structure

```
tests/
├── wrapper_tests/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── test_nom_wrapper.rs
│   │   ├── test_hashbrown_wrapper.rs
│   │   └── test_alloc_wrapper.rs
│   └── generated/  # symlink to output1 generated code
```

## Benefits

- **Incremental validation** - test each wrapper individually
- **Fast feedback** - don't wait for full bootstrap
- **Isolation** - debug wrapper issues without bootstrap complexity
- **Feature flags** - gradually enable wrappers as they're validated
