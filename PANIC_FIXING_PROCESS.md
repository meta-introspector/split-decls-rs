# Panic Fixing Process for split-decls-rs

This document outlines the systematic process for fixing panics in the split-decls-rs bootstrap process.

## Process Overview

1. **Identify Top Panics**: Use `grep -E "panic" build.log | sort | uniq -c | sort -rn | grep decl | head`
2. **Analyze Panic Locations**: Find the specific file and line causing panics
3. **Understand Root Cause**: Examine what code constructs are causing the panic
4. **Implement Fix**: Replace panic with graceful handling
5. **Test & Commit**: Verify fix works and commit with descriptive message

## Fixed Issues (December 27, 2025)

### 1. Module Declaration Panics ✅
**Location**: `declaration_extractor.rs:166`  
**Issue**: `Item::Mod` declarations caused panics  
**Examples**: `canonicalizer`, `prelude`, `alloc_error_handler`, `assert_dep_graph`, `layout`  
**Fix**: Generate module wrappers using `extracted_decl_base(mod_name, "mod".to_string())`  
**Commit**: `4008109c`

### 2. Missing Module Files ✅  
**Location**: `process_module_recursivly.rs:49`  
**Issue**: Module declarations without corresponding files caused panics  
**Examples**: `analysis`, `canonicalizer`, `parse`  
**Fix**: Generate warning and empty wrapper instead of panic  
**Commit**: `d2642964`

### 3. Extern Crate Declarations ✅
**Location**: `declaration_extractor.rs:171`  
**Issue**: `extern crate` statements not handled  
**Examples**: `#[cfg(feature = "alloc")] extern crate alloc;`, `extern crate self as rustc_hir;`  
**Fix**: Add `Item::ExternCrate` handling with `"extern_crate"` kind  
**Commit**: `d2642964`

## Next Targets

Run the command to find the next batch of panics:
```bash
grep -E "panic" build.log | sort | uniq -c | sort -rn | grep decl | head
```

## Pattern for Fixes

1. **Replace panics with warnings**: Use `eprintln!("WARNING: ...")` instead of `panic!(...)`
2. **Generate fallback wrappers**: Use `extracted_decl_base()` or similar to create placeholder content
3. **Handle gracefully**: Allow bootstrap to continue processing other crates
4. **Preserve information**: Log what was skipped for later analysis

## Testing Strategy

- Build after each fix: `cargo build`
- Quick bootstrap test: `timeout 60s cargo run --bin split-decls-rs -- bootstrap`
- Check for remaining panics: `grep -E "panic" new_build.log`

## Success Metrics

- **Panic reduction**: Each fix should eliminate a category of panics
- **Bootstrap progress**: More crates should process successfully
- **No regressions**: Previous fixes should remain working

## Documentation

Each commit should include:
- Clear description of what was fixed
- Examples of the problematic constructs
- How the fix enables further progress
- Reference to the specific panic locations

This systematic approach enables rapid iteration and measurable progress toward 100% bootstrap success.
