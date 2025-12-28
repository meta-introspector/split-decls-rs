# Makefile for minimal-test traced bootstrap execution
# Collects build and run errors for analysis

.PHONY: all build run clean logs help errors-only

# Default target
all: build run

# Build with error collection (warnings filtered)
build:
	@echo "🔧 Building traced bootstrap system..."
	cargo build 2>&1 | grep -E "(error|Error|failed|Failed)" | tee build_errors.log
	@cargo build >/dev/null 2>&1 && echo "✅ Build successful" || echo "❌ Build failed - see build_errors.log"

# Run with error collection
run:
	@echo "🚀 Running traced bootstrap execution..."
	cargo run 2>&1 | grep -E "(error|Error|failed|Failed|panic)" | tee run_errors.log
	@cargo run >/dev/null 2>&1 && echo "✅ Run successful" || echo "❌ Run failed - see run_errors.log"

# Build and run in sequence
test: build run

# Clean build artifacts and logs
clean:
	cargo clean
	rm -f *_errors.log *.log

# Show only errors (no warnings)
errors-only:
	@echo "📋 Build errors only:"
	@cat build_errors.log 2>/dev/null || echo "No build errors found"
	@echo "\n📋 Run errors only:"
	@cat run_errors.log 2>/dev/null || echo "No run errors found"

# Full build with all output
full-build:
	@echo "🔧 Full build with all output..."
	cargo build 2>&1 | tee full_build.log

# Count errors
count-errors:
	@echo "📊 Error counts:"
	@echo "Build errors: $$(wc -l < build_errors.log 2>/dev/null || echo 0)"
	@echo "Run errors: $$(wc -l < run_errors.log 2>/dev/null || echo 0)"

# Help
help:
	@echo "Available targets:"
	@echo "  build        - Build with error logging (no warnings)"
	@echo "  run          - Run with error logging"
	@echo "  test         - Build then run"
	@echo "  clean        - Clean artifacts and logs"
	@echo "  errors-only  - Show only errors (no warnings)"
	@echo "  full-build   - Build with all output including warnings"
	@echo "  count-errors - Count number of errors"
	@echo "  help         - Show this help"
