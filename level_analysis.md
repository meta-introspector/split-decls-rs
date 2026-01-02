# Rustc Crate Level Analysis

Progressive dependency analysis for optimal build ordering and usage pattern extraction.

## Level 0: Independent Crates (35 total)

These crates have zero dependencies and can be built in parallel:

### Core Infrastructure (9 crates)
- `rustc_middle` - Central IR and type system foundation
- `rustc_macros` - Procedural macro definitions  
- `rustc_data_structures` - Core data structures
- `rustc_session` - Compilation session management
- `rustc_hir` - High-level intermediate representation
- `rustc_ast` - Abstract syntax tree definitions
- `rustc_serialize` - Serialization utilities
- `rustc_index` - Index types and utilities
- `rustc_arena` - Memory arena allocation

### Utilities (8 crates)
- `rustc_fs_util` - File system utilities
- `rustc_lexer` - Lexical analysis
- `rustc_graphviz` - Graph visualization
- `rustc_log` - Logging infrastructure
- `rustc_thread_pool` - Thread pool management
- `rustc_error_codes` - Error code definitions
- `rustc_baked_icu_data` - Internationalization data
- `rustc_llvm` - LLVM bindings

### Analysis & Transform (10 crates)
- `rustc_infer` - Type inference engine
- `rustc_trait_selection` - Trait resolution
- `rustc_pattern_analysis` - Pattern matching analysis
- `rustc_expand` - Macro expansion
- `rustc_ast_passes` - AST transformation passes
- `rustc_ast_pretty` - AST pretty printing
- `rustc_metadata` - Crate metadata handling
- `rustc_interface` - Compiler interface
- `rustc_driver` - Driver coordination
- `rustc_attr_parsing` - Attribute parsing

### Macro & Format (8 crates)
- `rustc_fluent_macro` - Fluent localization macros
- `rustc_parse_format` - Format string parsing
- `rustc_index_macros` - Index-related macros
- `rustc_type_ir_macros` - Type IR macros
- `rustc_next_trait_solver` - Next-gen trait solver
- `rustc_sanitizers` - Sanitizer support
- `rustc` - Main rustc binary

## Level 1: Single Dependency Crates (11 total)

Each depends on exactly one Level 0 crate:

### rustc_middle Dependents (7 crates)
- `rustc_mir_build` → `rustc_middle`
- `rustc_traits` → `rustc_middle`  
- `rustc_incremental` → `rustc_middle`
- `rustc_const_eval` → `rustc_middle`
- `rustc_mir_dataflow` → `rustc_middle`
- `rustc_passes` → `rustc_middle`
- `rustc_ty_utils` → `rustc_middle`

### Other Dependencies (4 crates)
- `rustc_hashes` → `rustc_stable_hash`
- `rustc_type_ir` → `rustc_macros`
- `rustc_feature` → `rustc_span`
- `rustc_public` → `rustc_public_bridge`

## Build Strategy

### Phase 1: Level 0 Analysis
1. Run usage analysis on all 35 independent crates
2. Extract symbol definitions and usage patterns
3. Build foundation symbol database
4. Results feed into Level 1 analysis

### Phase 2: Level 1 Analysis  
1. Reuse Level 0 results as input
2. Analyze 11 single-dependency crates
3. Combine with Level 0 patterns
4. Propagate to higher levels

### Key Insight
`rustc_middle` is the critical foundation with 7 Level 1 dependents, confirming it as the optimal bootstrap starting point for incremental compilation.

## Next Steps
1. Execute usage analysis on Level 0 crates
2. Document symbol extraction results
3. Progress to Level 1 with accumulated knowledge
4. Continue depth-first through dependency tree
