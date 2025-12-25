.PHONY: build scanner repl clean

# Use sccache for faster builds
export RUSTC_WRAPPER=sccache

# Build all binaries once
build:
	cargo build --bins

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

# Run traced bootstrap with CFT proof generation
run_bootstrap:
	../../target/debug/bootstrap_tracer

# Run stateful REPL directly  
repl:
	../../target/debug/stateful_repl

# Clean build artifacts
clean:
	cargo clean
	sccache --zero-stats
