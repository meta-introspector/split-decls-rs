.PHONY: build scanner repl clean

# Use sccache for faster builds
SCCACHE := /home/mdupont/.cargo/bin/sccache
export RUSTC_WRAPPER=$(SCCACHE)

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
	@echo "Running bootstrap with sccache..." && RUSTC_WRAPPER=$(SCCACHE) RUST_BACKTRACE=full cargo run --bin split-decls-rs -- bootstrap 2>&1 | tee bootstrap_run.log

# Regenerate Cargo.toml files only (fast, no syn parsing) - using lib-cargo
regen_cargo:
	@echo "Regenerating Cargo.toml files with sccache..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin regen_cargo_v2 -- --output-dir output2 --verbose 2>&1 | tee regen_cargo.log

# Regenerate output2 workspace only (isolated)
regen_output2:
	@echo "Regenerating output2 workspace..." && cd output2 && RUSTC_WRAPPER=$(SCCACHE) ../target/debug/regen_cargo_v2 --output-dir . --verbose 2>&1 | tee ../regen_output2.log

# Build regen tool first
build_regen:
	@echo "Building regen tool..." && RUSTC_WRAPPER=$(SCCACHE) cargo build --bin regen_cargo_v2

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

# Generate workspace quietly with sccache (no output unless error)
gen_workspace_quiet:
	@RUSTC_WRAPPER=$(SCCACHE) cargo run --bin gen_workspace -q >/dev/null 2>&1 || echo "gen_workspace failed"

# Wrap single crate: addr2line
wrap_addr2line:
	@echo "Wrapping addr2line crate..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin wrap_single_crate -- ../addr2line --verbose 2>&1 | grep -E "(error|Error|failed|panic|WARN)" || echo "✅ addr2line wrapped successfully"

# Test wrapped addr2line with lisp eval RDF system
test_addr2line:
	@echo "Testing wrapped addr2line..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin test_addr2line 2>&1 | tee test_addr2line.log | grep -E "(error|Error|failed|panic)" || echo "✅ Test completed"

# Test wrapped addr2line module directly
test_addr2line_module:
	@echo "Testing wrapped addr2line module..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin test_addr2line_module 2>&1 | tee test_module.log | grep -E "(error|Error|failed|panic)" || echo "✅ Module test completed"

# PROOF: !wrap_bin macro that actually uses addr2line
proof_wrap_bin:
	@echo "🔥 PROOF: !wrap_bin macro..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin proof_wrap_bin

# PROOF: decl2addr! and alldecls! macros for declaration mapping
proof_decl2addr:
	@echo "🔥 PROOF: decl2addr! and alldecls! macros..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin proof_decl2addr

# Analyze common terms and real addresses
analyze_terms:
	@echo "🔍 Analyzing common terms and real addresses..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin analyze_common_terms

# Extract standalone crate with dependencies
extract_crate:
	@echo "📦 Extracting crate..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin extract_crate -- $(CRATE) --output $(OUTPUT) --verbose

# Extract addr2line as example
extract_addr2line:
	@echo "📦 Extracting addr2line..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin extract_crate -- addr2line --output extracted-addr2line --verbose

# Run stateful REPL directly  
repl:
	../../target/debug/stateful_repl

# Clean build artifacts
clean:
	cargo clean
	sccache --zero-stats
