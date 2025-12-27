# Makefile SOPs - split-decls-rs

## Overview
This document provides Standard Operating Procedures (SOPs) for using the Makefile in the split-decls-rs project. The Makefile automates common development tasks including building, bootstrapping, testing, and workspace management.

## Prerequisites
- Rust toolchain installed
- sccache installed at `/home/mdupont/.cargo/bin/sccache`
- Project cloned and in the root directory

## Core Operations

### 1. Building

#### Build All Binaries
```bash
make build
```
**Purpose**: Compiles all project binaries using sccache for faster builds
**When to use**: Initial setup, after major changes
**Output**: Compiled binaries in `target/debug/`

#### Build Specific Components
```bash
make build_output2_wrapper    # Build output2-wrapper specifically
make build_regen             # Build regen tool only
```

### 2. Bootstrap Operations

#### Standard Bootstrap
```bash
make run_bootstrap
```
**Purpose**: Runs the main bootstrap process with full logging
**Output**: `bootstrap_run.log` with complete execution trace
**Duration**: ~10-30 minutes depending on codebase size
**When to use**: Initial project setup, testing overlay system

#### Audited Bootstrap (Security)
```bash
make run_audited_bootstrap
```
**Purpose**: Runs bootstrap with full syscall tracking for security auditing
**Output**: `audited_bootstrap_run.log` with syscall analysis
**When to use**: Security validation, compliance checks

### 3. Workspace Management

#### Regenerate Cargo.toml Files
```bash
make regen_cargo             # Fast regeneration (no syn parsing)
make regen_output2           # Regenerate output2 workspace only
make regen_build             # Regenerate and test build
```

#### Workspace Generation
```bash
make gen_workspace           # Generate workspace with verbose output
make gen_workspace_quiet     # Generate workspace silently
```

#### Dependency Management
```bash
make audit_deps              # Audit all dependencies
make fix_deps                # Fix dependency issues automatically
```

### 4. Single Crate Operations

#### Wrap Individual Crates
```bash
make wrap_addr2line          # Example: wrap addr2line crate
```

#### Test Wrapped Crates
```bash
make test_addr2line          # Test wrapped addr2line
make test_addr2line_module   # Test addr2line module directly
```

#### Extract Standalone Crates
```bash
make extract_addr2line       # Extract addr2line as standalone
make extract_crate CRATE=<name> OUTPUT=<dir>  # Extract any crate
```

### 5. Analysis and Proof Tools

#### Mathematical Analysis
```bash
make ktheory                 # Run K-theory dependency analysis
make indexer                 # Run K-theory indexer
make lmfdb                   # Query LMFDB database
make demo                    # Run math similarity demo
```

#### Proof Systems
```bash
make proof                   # Run Lean4 proof system
make proof-quiet             # Run proof system quietly
```

#### Code Analysis
```bash
make analyze_terms           # Analyze common terms and addresses
make proof_wrap_bin          # Test !wrap_bin macro
make proof_decl2addr         # Test decl2addr! and alldecls! macros
```

### 6. Testing and Validation

#### Library Testing
```bash
make test_lib_cargo          # Test lib-cargo system
```

#### Evaluation
```bash
make eval_main               # Evaluate wrapped split-decls-rs main
make run_enhanced            # Run enhanced generation
```

### 7. Interactive Tools

#### REPL
```bash
make repl                    # Run stateful REPL
```

### 8. Maintenance

#### Clean Up
```bash
make clean                   # Clean build artifacts and reset sccache stats
```

## Common Workflows

### Initial Setup Workflow
1. `make build` - Build all binaries
2. `make run_bootstrap` - Run initial bootstrap
3. Check `bootstrap_run.log` for issues
4. `make gen_workspace` - Generate workspace

### Development Workflow
1. `make build` - Build changes
2. `make wrap_addr2line` - Test with single crate
3. `make test_addr2line` - Validate wrapping
4. `make regen_cargo` - Update workspace if needed

### Debugging Workflow
1. `make run_bootstrap` - Run with full logging
2. Analyze `bootstrap_run.log`
3. `make wrap_addr2line` - Test problematic crate individually
4. `make fix_deps` - Fix dependency issues
5. `make audit_deps` - Verify fixes

### Security Audit Workflow
1. `make run_audited_bootstrap` - Run with syscall tracking
2. Review `audited_bootstrap_run.log`
3. `make audit_deps` - Check dependency security
4. `make proof` - Run formal verification

## Troubleshooting

### Common Issues

#### Build Failures
- **Solution**: `make clean && make build`
- **Check**: sccache is installed and accessible

#### Bootstrap Failures
- **Check**: `bootstrap_run.log` for specific errors
- **Solution**: `make fix_deps` then retry
- **Alternative**: Test with single crate first

#### Dependency Issues
- **Solution**: `make audit_deps` then `make fix_deps`
- **Check**: All submodules are properly initialized

#### Workspace Problems
- **Solution**: `make gen_workspace` to regenerate
- **Check**: Root Cargo.toml has correct workspace configuration

### Log Files
- `bootstrap_run.log` - Main bootstrap execution
- `audited_bootstrap_run.log` - Security audit results
- `regen_cargo.log` - Cargo.toml regeneration
- `test_addr2line.log` - Single crate test results

## Performance Notes

- All operations use sccache for faster compilation
- Bootstrap operations are CPU-intensive (expect 10-30 min)
- Quiet operations suppress output unless errors occur
- Regeneration operations are faster than full bootstrap

## Safety Considerations

- Always review logs after bootstrap operations
- Use audited bootstrap for security-sensitive environments
- Test single crates before full workspace operations
- Keep backups before running fix operations

## Integration with Development

These Makefile targets integrate with the broader development workflow:
- Use with git hooks for automated testing
- Integrate with CI/CD pipelines
- Combine with monitoring for production deployments
- Use quiet operations in automated scripts
