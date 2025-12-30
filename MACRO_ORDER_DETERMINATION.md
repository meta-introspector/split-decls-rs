# Macro Set and Order Determination

## Overview
The `generate_import_macros.rs` tool determines both the **set** and **order** of macros for the `mkbin!()` macro based on the recursive dependency resolution results.

## Order Determination Process

### 1. Source of Order: `recursive_dependencies.json`
- **Created by**: `recursive_resolver.rs`
- **Algorithm**: Breadth-First Search (BFS) traversal
- **Starting Points**: Initial rustc terms:
  ```rust
  let initial_terms = vec![
      "get_resident_set_size", "TimePassesCallbacks", "install_ice_hook", 
      "run_compiler", "main", "init_rustc_env_logger", "catch_with_exit_code"
  ];
  ```

### 2. BFS Traversal Logic
```rust
while let Some(current_term) = queue.pop_front() {
    if resolved.contains(&current_term) {
        continue; // Skip already processed
    }
    
    resolved.insert(current_term.clone());
    
    // Extract dependencies from current term's file
    if let Ok(deps) = extract_dependencies_from_file(file_path) {
        // Add new dependencies to end of queue
        for dep in deps {
            if !resolved.contains(&dep) {
                queue.push_back(dep);
            }
        }
    }
}
```

### 3. Order Characteristics
- **Breadth-First**: Dependencies at same level processed before deeper levels
- **Deduplication**: Each dependency appears only once (first occurrence wins)
- **Dependency-Driven**: Order follows actual code dependency relationships
- **Deterministic**: Same input produces same order

## Set Determination

### 1. Inclusion Criteria
A dependency is included if:
- **Reachable**: Found via BFS from initial rustc terms
- **Resolvable**: Has entry in `name_index.json`
- **Parseable**: File can be read and parsed for further dependencies

### 2. Exclusion Cases
Dependencies excluded if:
- **Not found**: No entry in name index
- **Unreachable**: No path from initial terms
- **Parse errors**: File cannot be processed

### 3. Current Results
- **Total Set**: 12,511 unique dependencies
- **Coverage**: 100% of reachable dependencies from rustc
- **Success Rate**: 100% resolution (up from 3.1% with basic parsing)

## Macro Generation Order

### 1. Import Macros (`import_macros.rs`)
```rust
// Generated in BFS order from recursive_dependencies.json
for term in resolved_terms {
    macro_rules! import_{safe_name} {
        () => { include!("../../output2/{file_path}"); };
    }
}
```

### 2. mkbin Macro Structure
```rust
macro_rules! mkbin {
    () => {
        // Modules generated in same BFS order
        mod {safe_name_1} { import_{safe_name_1}!(); }
        mod {safe_name_2} { import_{safe_name_2}!(); }
        // ... all 12,511 dependencies in BFS order
    };
}
```

## Key Files and Their Roles

| File | Role in Order/Set |
|------|-------------------|
| `recursive_resolver.rs` | **Determines order** via BFS traversal |
| `recursive_dependencies.json` | **Stores ordered set** of dependencies |
| `generate_import_macros.rs` | **Preserves order** in macro generation |
| `name_index.json` | **Filters set** (only indexed names included) |

## Order Properties

### Advantages of BFS Order
1. **Dependencies First**: Required dependencies loaded before dependents
2. **Level-by-Level**: Clear dependency layers
3. **Minimal Forward References**: Reduces circular dependency issues
4. **Predictable**: Same traversal produces same order

### Current Issue
- **Order Generated**: ✅ 12,511 dependencies in BFS order
- **Order Preserved**: ✅ Same order in import_macros.rs
- **Order Executed**: ❌ Only 11 dependencies executed in mkbin

## Fix Required
Update mkbin generation to use the **complete ordered set** from `recursive_dependencies.json` instead of just the initial 7 rustc terms, ensuring all 12,511 dependencies are executed in the correct BFS order.
