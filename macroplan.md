# Macro Plan: mkdecl System Enhancement

## Current Status ✅

- **mkdecl wrappers implemented**: Functions wrapped with `mkdeclfn!`, structs with `mkdeclstruct!`, etc.
- **Bootstrap3 integration working**: Uses extracted functions with mkdecl wrappers
- **Recursive generation**: Bootstrap3 → simple splitter → output3 with macro support
- **Infrastructure ready**: Wrapper system in place for enhancement

## Goal 🎯

Add function call tracing to `mkdeclfn!` macro so every function call prints:
```
🔧 Calling function: setup_crate_paths
```

## Technical Approach

### Phase 1: Syn-based Function Modification

Use the `VisitMut` pattern found in output2 to inject println statements:

```rust
macro_rules! mkdeclfn {
    ($($content:tt)*) => {
        {
            use syn::{parse_quote, ItemFn, Stmt};
            let mut item: ItemFn = parse_quote! { $($content)* };
            
            // Extract function name
            let fn_name = &item.sig.ident;
            
            // Create println statement
            let println_stmt: Stmt = parse_quote! {
                println!("🔧 Calling function: {}", stringify!(#fn_name));
            };
            
            // Insert at start of function body
            item.block.stmts.insert(0, println_stmt);
            
            quote! { #item }
        }
    };
}
```

### Phase 2: Simple Splitter Integration

Update simple splitter to generate functions ready for macro enhancement:
- Keep current mkdecl wrapper generation
- Ensure function format is parseable by syn
- Test with bootstrap3 integration

### Phase 3: Advanced Tracing

Extend to other declaration types:
- `mkdeclstruct!` - Constructor tracing
- `mkdeclimpl!` - Method call tracing  
- `mkdeclenum!` - Variant access tracing

## Implementation Steps

1. **Test syn parsing** on extracted functions from output2
2. **Create working mkdeclfn macro** with println injection
3. **Verify bootstrap3 integration** shows function call traces
4. **Extend to other mkdecl types** as needed
5. **Document macro system** for future enhancements

## Reference Code

Found in output2: `ExpressionReplacer` using `VisitMut` pattern:
- `visit_item_fn_mut` - Modify function items
- `visit_block_mut` - Modify function bodies
- Pattern for AST transformation with syn

## Success Criteria

When running bootstrap3, see output like:
```
🔧 Calling function: setup_crate_paths
🔍 CHECKING PATHS for crate: test
🔧 Calling function: process_crates_in_path
DRY RUN: Would process crates in .
```

## Next Action

Implement Phase 1 using the syn parsing approach with the VisitMut pattern from output2.
