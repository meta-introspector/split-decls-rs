.PHONY: build scanner repl clean perf_record perf_report perf_functions

# Use sccache for faster builds
SCCACHE := /home/mdupont/.cargo/bin/sccache
export RUST_BACKTRACE=FULL
export RUSTC_WRAPPER=$(SCCACHE)

# Build all binaries once
build:
	RUSTC_WRAPPER=$(SCCACHE) cargo build --bins

# Build split-decls-rs specifically  
build_split_decls:
	RUSTC_WRAPPER=$(SCCACHE) cargo build --bin split-decls-rs

# Build output2-wrapper specifically
build_output2_wrapper:
	cargo build --bin output2-wrapper

# Step 1: Update split-decls-rs.toml configuration
step1_update_config:
	@echo "Step 1: Updating configuration..."
	@cargo run --bin update_split_decls_config -- --split-decls-config-path split-decls-rs.toml > step1_update_config.log 2>&1
	@echo "✅ Step 1 complete - logs saved to step1_update_config.log"

# Step 2: Run bootstrap process
step2_bootstrap:
	@echo "Step 2: Running bootstrap..."
	@cargo run --bin split-decls-rs -- bootstrap > step2_bootstrap.log 2>&1
	@echo "✅ Step 2 complete - logs saved to step2_bootstrap.log"

# Step 3: Build output2 workspace
step3_build_output2:
	@echo "Step 3: Building output2 workspace..."
	@cd output2 && cargo build > ../step3_build_output2.log 2>&1
	@echo "✅ Step 3 complete - logs saved to step3_build_output2.log"

# Chain all steps together
bootstrap_chain_steps: step1_update_config step2_bootstrap step3_build_output2
	@echo "=== BOOTSTRAP CHAIN COMPLETE ==="
	@echo "All logs saved to: step1_update_config.log, step2_bootstrap.log, step3_build_output2.log"
	@echo "=== SUMMARY ==="
	@echo "Step 1 errors:" && (grep -E "(error|Error|failed|Failed)" step1_update_config.log | head -5 || echo "No errors")
	@echo "Step 2 errors:" && (grep -E "(error|Error|failed|Failed)" step2_bootstrap.log | head -5 || echo "No errors") 
	@echo "Step 3 errors:" && (grep -E "(error|Error|failed|Failed)" step3_build_output2.log | head -5 || echo "No errors")

# Legacy targets (keep for compatibility)
update_config: step1_update_config
bootstrap_chain: bootstrap_chain_steps
bootstrap_quiet: bootstrap_chain_steps

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

# Run bootstrap with split-decls-rs (no building)
run_bootstrap:
#	@echo "Running bootstrap (no build)..." && RUST_BACKTRACE=FULL cargo run --bin split-decls-rs -- bootstrap 2>&1 | tee bootstrap_run.log
	@echo "Running bootstrap (no build)..." && RUST_BACKTRACE=FULL cargo run --bin split-decls-rs -- bootstrap  > bootstrap_run.log 2>&1

# Run AUDITED bootstrap with full syscall tracking
run_audited_bootstrap:
	@echo "Running AUDITED bootstrap with full syscall tracking..." && RUSTC_WRAPPER=$(SCCACHE) RUST_BACKTRACE=full cargo run --bin bootstrap-self-apply_audited 2>&1 | tee audited_bootstrap_run.log

# Regenerate Cargo.toml files only (fast, no syn parsing) - using lib-cargo
regen_cargo:
	@echo "Regenerating Cargo.toml files with sccache..." && RUSTC_WRAPPER=$(SCCACHE) cargo run --bin regen_cargo_v2 -- --output-dir output2 --verbose 2>&1 | tee regen_cargo.log

# Regenerate output2 workspace only (isolated)
regen_output2:
	@echo "Regenerating output2 workspace..." && cd output2 && RUSTC_WRAPPER=$(SCCACHE) ../target/debug/regen_cargo_v2 --output-dir . --verbose 2>&1 | tee ../regen_output2.log

# Build regen tool first
build_regen:
	@echo "Building regen tool..." && RUSTC_WRAPPER=$(SCCACHE) cargo build --bin regen_cargo_v2 --quiet

# Test the updated lib-cargo system
test_lib_cargo:
	@echo "Testing lib-cargo system..." && RUSTC_WRAPPER=$(SCCACHE) cargo check --bin regen_cargo_v2 --quiet
	@../../target/debug/regen_cargo_v2 --output-dir output2 --verbose --dry-run 2>&1 | head -15

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

perf_record:
	perf record -o bootstrap.perf -g ../../target/debug/split-decls-rs bootstrap

perf_report: bootstrap.perf
	perf report -i bootstrap.perf --stdio | grep -E "^\s*[0-9]+\.[0-9]+%" | head -20

perf_functions: bootstrap.perf
	perf report -i bootstrap.perf --stdio | grep -E "^\s*[0-9]+\.[0-9]+%" > perf_functions.txt
	@echo "Function execution data saved to perf_functions.txt"
eval_main:
	@echo "🎯 Evaluating wrapped split-decls-rs main..."
	@cargo run --bin eval_split_decl_main

run_enhanced:
	@echo "Running enhanced generation..."
	@RUSTC_WRAPPER=$(SCCACHE) cargo build --release --quiet
	@RUSTC_WRAPPER=$(SCCACHE) cargo run --release --bin generate_output3_from_enhanced --quiet 2>/dev/null || true

# Build specific package with errors only
build_pkg:
	@cargo build -p $(PKG) 2>$(PKG)_errors.log || true
	@echo "Build completed. Errors saved to $(PKG)_errors.log"

# Build smart compiler quietly and show only errors
build_smart_compiler:
	@cargo build --bin smart_compiler 2>smart_compiler_build.log || true
	@grep -E "(error|Error)" smart_compiler_build.log || echo "✅ No errors found"

# Run directory analysis quietly and show only errors
build_directory_analysis:
	@cargo build --bin directory_analysis 2>directory_analysis_build.log || true
	@grep -E "(error|Error)" directory_analysis_build.log || echo "✅ Directory analysis built"

# Run directory analysis
run_directory_analysis:
	@cargo run --bin directory_analysis 2>directory_analysis_run.log || true
	@cat directory_analysis_run.log

# Split syn crate using simple splitter
simple_split_syn:
	@echo "Splitting syn crate..." && cargo run --bin simple_split -- ../syn --output-dir output2 > simple_split_syn.log 2>&1 && echo "✅ syn split complete" || echo "❌ syn split failed"

# Split quote crate using simple splitter  
simple_split_quote:
	@echo "Splitting quote crate..." && cargo run --bin simple_split -- ../quote --output-dir output2 > simple_split_quote.log 2>&1 && echo "✅ quote split complete" || echo "❌ quote split failed"

# Split both syn and quote
simple_split_both: simple_split_syn simple_split_quote
	@echo "✅ Both syn and quote split complete"

# Split hir-ty with detailed output
simple_split_hir_ty:
	@echo "Splitting hir-ty crate..." && cargo run --bin simple_split -- ../rust-analyzer/crates/hir-ty --output-dir output2 > simple_split_hir_ty.log 2>&1 && echo "✅ hir-ty split complete" || echo "❌ hir-ty split failed"

# Split all crates from split-decls-rs.toml
simple_split_all:
	@echo "🚀 Splitting all crates from config..."
	@cargo run --bin simple_split_all_crates > simple_split_all.log 2>&1 && echo "✅ All crates split complete" || echo "❌ Some crates failed - check simple_split_all.log"
# Recursive Dependency Analysis System
recursive-analysis:
	@echo "🚀 Running Recursive Dependency Analysis System"
	@echo "================================================"
	@mkdir -p reports
	@cd incremental-bootstrap && \
	echo "📋 Discovering binaries..." && \
	cargo run -- list-bins > ../reports/binaries.txt 2>&1 && \
	echo "🔍 Analyzing simple_split dependencies..." && \
	cargo run -- analyze-deps --bin simple_split > ../reports/simple_split_deps.txt 2>&1 && \
	echo "🔄 Running recursive analysis (depth 2)..." && \
	cargo run -- recursive-deps --bin simple_split --depth 2 > ../reports/recursive_analysis.txt 2>&1 && \
	echo "📊 Generating dependency graph..." && \
	cargo run -- print-graph --bin simple_split --depth 2 > ../reports/dependency_graph.txt 2>&1 && \
	echo "🧪 Testing compilation..." && \
	cargo run -- test-eval --bin simple_split > ../reports/compilation_test.txt 2>&1
	@echo ""
	@echo "✅ Analysis Complete! Results saved to reports/"
	@echo "📁 Generated files:"
	@ls -la reports/
	@echo ""
	@echo "📋 Cache status:"
	@ls -la bootstrap3-incremental/dep-cache/ | head -5
	@echo ""
	@echo "📊 Summary Report:"
	@echo "=================="
	@echo "🔧 Binaries found: $$(grep -c '🔧' reports/binaries.txt || echo 'N/A')"
	@echo "📈 Dependencies resolved: $$(grep 'Total unique dependencies:' reports/recursive_analysis.txt | tail -1 | cut -d: -f2 | xargs || echo 'N/A')"
	@echo "🏗️  Cache entries: $$(grep 'Cache entries:' reports/recursive_analysis.txt | tail -1 | cut -d: -f2 | xargs || echo 'N/A')"
	@echo "✅ Compilation status: $$(if grep -q 'SUCCESS' reports/compilation_test.txt; then echo 'PASSED'; else echo 'FAILED (import conflicts)'; fi)"
	@echo ""
	@echo "📖 Full documentation: RECURSIVE_DEPENDENCY_ANALYSIS.md"

.PHONY: recursive-analysis
# Quick test targets for dependency resolution debugging

.PHONY: test-compile test-errors clean-test

# Quick compile test - shows first few errors
test-compile:
	@echo "🧪 Testing compilation with current mkbin..."
	@cargo run --bin rustc_traced_compile 2>&1 | head -20

# Full error analysis - saves to file and shows summary
test-errors:
	@echo "🔍 Running full error analysis..."
	@cargo run --bin rustc_traced_compile > test_output.log 2>&1 || true
	@echo "📊 Error Summary:"
	@grep -E "error\[" test_output.log | sort | uniq -c | sort -rn | head -10
	@echo "📁 Saved full output to test_output.log"

# Clean test artifacts
clean-test:
	@rm -f test_output.log rustc_*.log

# Quick bisection test - comment/uncomment imports and test
bisect-test:
	@echo "🔄 Quick bisection test..."
	@cargo run --bin rustc_traced_compile 2>&1 | grep -E "(error\[|Loading:)" | head -10
simple_split_all:
	cargo run --bin simple_split -- --config split-decls-rs.toml --recurse --jobs 40 > simple_split_all.log 2>&1
	@echo "📊 Processing complete. Results:"
	@grep "✅" simple_split_all.log | wc -l | xargs echo "✅ Success:"
	@grep "❌" simple_split_all.log | wc -l | xargs echo "❌ Errors:"
	@echo "📋 Top 10 error types:"
	@grep "❌" simple_split_all.log | sort | uniq -c | sort -rn | head -10
