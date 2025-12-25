.PHONY: build scanner repl clean

# Use sccache for faster builds
export RUSTC_WRAPPER=sccache

# Build all binaries once
build:
	cargo build --bins

# Run bootstrap scanner directly
scanner:
	../../target/debug/bootstrap_scanner

# Run stateful REPL directly  
repl:
	../../target/debug/stateful_repl

# Clean build artifacts
clean:
	cargo clean
	sccache --zero-stats
