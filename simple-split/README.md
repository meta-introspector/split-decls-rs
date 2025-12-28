# Simple Split - Minimal Working Split-Decls Tool

## Overview

This is a minimal, working replacement for the overcomplicated original split-decls-rs system. It successfully achieves self-application by processing split-decls-rs itself and extracting all functions into an organized structure.

## Key Features

- ✅ **Self-Application Achieved**: Successfully processed split-decls-rs itself
- ✅ **Parallel Processing**: Up to 20 crates processed in parallel
- ✅ **Organized Output**: Files organized as `decls/module/decltype/declsize/declname.rs`
- ✅ **Essential Functions Extracted**: Including `run_bootstrap_mode` function
- ✅ **Minimal Codebase**: ~150 lines vs thousands in original system

## Results

Successfully processed:
- **1 crate** (split-decls-rs)
- **13,575 files** scanned
- **16,946 items** extracted
- **Key function found**: `run_bootstrap_mode` in multiple locations

## Output Structure

```
output2/wrapped-unknown/src/decls/
├── main/fn/1191/run_bootstrap_mode.rs
├── wrapped_main_3/fn/2993/run_bootstrap_mode.rs
└── [module]/[type]/[size]/[name].rs
```

## Usage

```bash
cd simple-split
cargo run
```

## Next Steps

This tool will be used to bootstrap output3 generation, achieving the recursive self-generation goal that the original complex system failed to accomplish.

## Architecture

- **Parallel Crate Processing**: Up to 20 crates at once
- **Sequential File Processing**: Within each crate to avoid stack overflow
- **Smart Directory Filtering**: Skips output2 and enhanced_output
- **Organized Output**: Hierarchical structure for easy navigation
