# syn2macro: Universal AST Transformation Engine

## Vision

Convert any `syn`-based code into trait-based macros that can execute in multiple contexts:
- **Compiler context**: Direct rustc plugin execution
- **Syn context**: Traditional proc_macro execution  
- **Abstract engine**: Sandboxed execution with ACL/security
- **Remote execution**: Distributed AST processing

## Core Architecture

### 1. Universal AST Traits

```rust
pub trait UniversalAst {
    type TokenStream;
    type Error;
    
    fn parse_item(&self, input: Self::TokenStream) -> Result<Self::Item, Self::Error>;
    fn generate_code(&self, item: Self::Item) -> Self::TokenStream;
}

pub trait SecureExecution {
    fn check_permissions(&self, operation: &AstOperation) -> bool;
    fn sandbox_execute<F>(&self, f: F) -> Result<TokenStream, SecurityError>
    where F: FnOnce() -> TokenStream;
}
```

### 2. Context Adapters

```rust
// Compiler context
impl UniversalAst for CompilerContext {
    type TokenStream = rustc_ast::TokenStream;
    // ...
}

// Syn context  
impl UniversalAst for SynContext {
    type TokenStream = proc_macro2::TokenStream;
    // ...
}

// Abstract engine context
impl UniversalAst for AbstractEngine {
    type TokenStream = AbstractTokenStream;
    // ...
}
```

## Implementation Plan

### Phase 1: Core Trait System
- Define universal AST traits
- Create context adapters for syn/rustc
- Basic macro conversion framework

### Phase 2: Security Layer
- ACL system for AST operations
- Sandboxed execution environment
- Permission checking for transformations

### Phase 3: Abstract Engine
- Custom AST representation
- Distributed execution capability
- Cross-context serialization

## Immediate Next Steps

1. Create `src/bin/syn2macro.rs` - Main conversion tool
2. Design universal trait system
3. Implement syn → trait conversion
4. Add security/ACL framework
