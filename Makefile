.PHONY: build scanner repl clean

# Use sccache for faster builds
export RUSTC_WRAPPER=sccache

# Build all binaries once
build:
	cargo build --bins

# Build output2-wrapper specifically
build_output2_wrapper:
	cargo build --bin output2-wrapper

# Run 8-level K-theory dependency analysis
ktheory:
	../../target/debug/k_theory_deps

indexer:
	../../target/debug/k_theory_indexer

lmfdb:
	../../target/debug/lmfdb_query

demo:
	../../target/debug/math_similarity_demo

proof:
	../../target/debug/lean4_proof_system_simple

proof-quiet:
	@cargo build --bin lean4_proof_system_simple -q 2>/dev/null || echo "Build failed"
	@../../target/debug/lean4_proof_system_simple 2>/dev/null || echo "Execution failed"

# Run bootstrap with split-decls-rs
run_bootstrap:
	@echo "Running bootstrap..." && RUSTC_WRAPPER=$(SCCACHE) RUST_BACKTRACE=full cargo run --bin split-decls-rs -- bootstrap > bootstrap_run.log 2>&1

# Regenerate Cargo.toml files only (fast, no syn parsing)
regen_cargo:
	@echo "Regenerating Cargo.toml files..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin regen_cargo -q -- --verbose 2>/dev/null

# Regenerate and build test Cargo.toml files
regen_build:
	@echo "Regenerating and testing Cargo.toml files..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin regen_cargo -q -- --build 2>/dev/null

# Audit all dependencies to ensure they resolve to local submodules
audit_deps:
	@echo "Auditing dependencies..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin audit_deps -q -- --verbose 2>/dev/null

# Fix dependency issues automatically
fix_deps:
	@echo "Fixing dependencies..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin audit_deps -q -- --fix --verbose 2>/dev/null

# Generate proper workspace Cargo.toml with all deps from Cargo.lock
gen_workspace:
	@echo "Generating workspace..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin gen_workspace -q 2>/dev/null

# Run stateful REPL directly  
repl:
	../../target/debug/stateful_repl

# Clean build artifacts
clean:
	cargo clean
	sccache --zero-stats
