# Makefile Documentation

This document provides an overview and detailed explanation of the targets and variables defined in the project's `Makefile`. This `Makefile` is primarily used to automate common development and testing tasks, especially those related to building, regenerating, and auditing Rust crates and their dependencies within the `split-decls-rs` project.

## Variables

### `SCCACHE`
- **Purpose:** Specifies the path to the `sccache` binary, which is a ccache-like tool for Rust that can speed up compilation by caching compilation artifacts.
- **Value:** `/home/mdupont/.cargo/bin/sccache`
- **Usage:** This variable is exported as `RUSTC_WRAPPER`, making `sccache` the default Rust compiler wrapper for all `cargo` commands executed within the `Makefile`.

## Targets

### `build`
- **Purpose:** Builds all binary crates defined in the project.
- **Command:** `cargo build --bins`

### `build_output2_wrapper`
- **Purpose:** Specifically builds the `output2-wrapper` binary.
- **Command:** `cargo build --bin output2-wrapper`

### `ktheory`
- **Purpose:** Runs an 8-level K-theory dependency analysis.
- **Command:** `../../target/debug/k_theory_deps`

### `indexer`
- **Purpose:** Runs the K-theory indexer.
- **Command:** `../../target/debug/k_theory_indexer`

### `lmfdb`
- **Purpose:** Executes an LMFDB (L-functions and Modular Forms Database) query.
- **Command:** `../../target/debug/lmfdb_query`

### `demo`
- **Purpose:** Runs the math similarity demonstration.
- **Command:** `../../target/debug/math_similarity_demo`

### `proof`
- **Purpose:** Runs the Lean4 proof system in simple mode.
- **Command:** `../../target/debug/lean4_proof_system_simple`

### `proof-quiet`
- **Purpose:** Builds and runs the Lean4 proof system quietly, suppressing verbose output unless there's a failure.
- **Command:** `@cargo build --bin lean4_proof_system_simple -q 2>/dev/null || echo "Build failed"`
  `@../../target/debug/lean4_proof_system_simple 2>/dev/null || echo "Execution failed"`

### `run_bootstrap`
- **Purpose:** Executes the bootstrap process using `split-decls-rs` with `sccache` enabled and full backtrace on errors.
- **Command:** `@echo "Running bootstrap with sccache..." && RUSTC_WRAPPER=$(SCCACHE) RUST_BACKTRACE=full cargo run --bin split-decls-rs -- bootstrap 2>&1 | tee bootstrap_run.log`

### `run_audited_bootstrap`
- **Purpose:** Runs an "AUDITED" bootstrap process with full syscall tracking enabled.
- **Command:** `@echo "Running AUDITED bootstrap with full syscall tracking..." && RUSTC_WRAPPER=$(SCCACHE) RUST_BACKTRACE=full cargo run --bin bootstrap-self-apply_audited 2>&1 | tee audited_bootstrap_run.log`

### `regen_cargo`
- **Purpose:** Regenerates `Cargo.toml` files using `lib-cargo` (fast, no `syn` parsing).
- **Command:** `@echo "Regenerating Cargo.toml files with sccache..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin regen_cargo_v2 -- --output-dir output2 --verbose 2>&1 | tee regen_cargo.log`

### `regen_output2`
- **Purpose:** Regenerates the `output2` workspace in isolation.
- **Command:** `@echo "Regenerating output2 workspace..." && cd output2 && RUSTC_WRAPPER=$(SCCACHE) ../target/debug/regen_cargo_v2 --output-dir . --verbose 2>&1 | tee ../regen_output2.log`

### `build_regen`
- **Purpose:** Builds the `regen_cargo_v2` tool.
- **Command:** `@echo "Building regen tool..." && RUSTC_WRAPPER=$(SCCACHE) cargo build --bin regen_cargo_v2 --quiet`

### `test_lib_cargo`
- **Purpose:** Tests the updated `lib-cargo` system by checking the `regen_cargo_v2` binary and then running a dry-run regeneration.
- **Command:** `@echo "Testing lib-cargo system..." && RUSTC_WRAPPER=$(SCCACHE) cargo check --bin regen_cargo_v2 --quiet`
  `@../../target/debug/regen_cargo_v2 --output-dir output2 --verbose --dry-run 2>&1 | head -15`

### `regen_build`
- **Purpose:** Regenerates and builds test `Cargo.toml` files quietly.
- **Command:** `@echo "Regenerating and testing Cargo.toml files..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin regen_cargo -q -- --build 2>/dev/null`

### `audit_deps`
- **Purpose:** Audits all project dependencies to ensure they resolve to local submodules.
- **Command:** `@echo "Auditing dependencies..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin audit_deps -q -- --verbose 2>/dev/null`

### `fix_deps`
- **Purpose:** Automatically fixes dependency issues identified by the audit.
- **Command:** `@echo "Fixing dependencies..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin audit_deps -q -- --fix --verbose 2>/dev/null`

### `gen_workspace`
- **Purpose:** Generates a proper workspace `Cargo.toml` file including all dependencies from `Cargo.lock`.
- **Command:** `@echo "Generating workspace..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin gen_workspace -q 2>/dev/null`

### `gen_workspace_quiet`
- **Purpose:** Generates the workspace quietly, suppressing output unless an error occurs.
- **Command:** `@RUSTC_WRAPPER=$(SCCACHE) cargo run --bin gen_workspace -q >/dev/null 2>&1 || echo "gen_workspace failed"`

### `wrap_addr2line`
- **Purpose:** Wraps the `addr2line` crate.
- **Command:** `@echo "Wrapping addr2line crate..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin wrap_single_crate -- ../addr2line --verbose 2>&1 | grep -E "(error|Error|failed|panic|WARN)" || echo "✅ addr2line wrapped successfully"`

### `test_addr2line`
- **Purpose:** Tests the wrapped `addr2line` crate with the lisp eval RDF system.
- **Command:** `@echo "Testing wrapped addr2line..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin test_addr2line 2>&1 | tee test_addr2line.log | grep -E "(error|Error|failed|panic)" || echo "✅ Test completed"`

### `test_addr2line_module`
- **Purpose:** Directly tests the wrapped `addr2line` module.
- **Command:** `@echo "Testing wrapped addr2line module..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin test_addr2line_module 2>&1 | tee test_module.log | grep -E "(error|Error|failed|panic)" || echo "✅ Module test completed"`

### `proof_wrap_bin`
- **Purpose:** Demonstrates the `!wrap_bin` macro which uses `addr2line`.
- **Command:** `@echo "🔥 PROOF: !wrap_bin macro..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin proof_wrap_bin`

### `proof_decl2addr`
- **Purpose:** Demonstrates the `decl2addr!` and `alldecls!` macros for declaration mapping.
- **Command:** `@echo "🔥 PROOF: decl2addr! and alldecls! macros..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin proof_decl2addr`

### `analyze_terms`
- **Purpose:** Analyzes common terms and real addresses.
- **Command:** `@echo "🔍 Analyzing common terms and real addresses..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin analyze_common_terms`

### `extract_crate`
- **Purpose:** Extracts a standalone crate along with its dependencies. Requires `CRATE` and `OUTPUT` environment variables to be set.
- **Command:** `@echo "📦 Extracting crate..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin extract_crate -- $(CRATE) --output $(OUTPUT) --verbose`

### `extract_addr2line`
- **Purpose:** Extracts `addr2line` as an example.
- **Command:** `@echo "📦 Extracting addr2line..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin extract_crate -- addr2line --output extracted-addr2line --verbose`

### `repl`
- **Purpose:** Runs the stateful REPL (Read-Eval-Print Loop) directly.
- **Command:** `../../target/debug/stateful_repl`

### `clean`
- **Purpose:** Cleans build artifacts and clears the `sccache` statistics.
- **Command:** `cargo clean`
  `sccache --zero-stats`

### `eval_main`
- **Purpose:** Evaluates the wrapped `split-decls-rs` main function.
- **Command:** `@echo "🎯 Evaluating wrapped split-decls-rs main..."`
  `@cargo run --bin eval_split_decl_main`

### `run_enhanced`
- **Purpose:** Runs an enhanced generation process, first building in release mode and then executing `generate_output3_from_enhanced`.
- **Command:** `@echo "Running enhanced generation..."`
  `@RUSTC_WRAPPER=$(SCCACHE) cargo build --release --quiet`
  `@RUSTC_WRAPPER=$(SCCACHE) cargo run --release --bin generate_output3_from_enhanced --quiet 2>/dev/null || true`
