SCCACHE := $(HOME)/.cargo/bin/sccache
# Use RUSTC_WRAPPER for sccache
CARGO := cargo

.PHONY: all build_core run_bootstrap build_output_module clean_output2 check_build_errors check_bootstrap_errors debug-main bootstrap_only check_bootstrap_only

all: debug-main run_bootstrap check_bootstrap_errors build_output_module

debug-main: build_core check_build_errors

build_core:
	-RUSTC_WRAPPER=$(SCCACHE) $(CARGO) build > build_core.log 2>&1 || true

check_build_errors:
	grep -E "error\[|error:" build_core.log || true

run_bootstrap: clean_output2
	@echo "Running bootstrap..." && RUSTC_WRAPPER=$(SCCACHE) RUST_BACKTRACE=full $(CARGO) run --bin split-decls-rs -- bootstrap > temp_bootstrap.log 2>&1 && echo "Bootstrap completed successfully" || echo "Bootstrap finished"

check_bootstrap_errors:
	grep -E "error\[|error:" temp_bootstrap.log || true

bootstrap_only:
	@echo "Running bootstrap..."
	@RUSTC_WRAPPER=$(SCCACHE) RUST_BACKTRACE=full $(CARGO) run --bin split-decls-rs -- bootstrap > bootstrap_output.log 2>&1 && echo "Bootstrap completed"
	@echo "Checking for errors..."
	@grep -E "error\[|error:" bootstrap_output.log || echo "No errors found in bootstrap"

check_bootstrap_only: bootstrap_only
	@echo "Checking for errors..." && grep -E "error\[|error:" bootstrap_output.log || echo "No errors found in bootstrap"



build_output_module:
	@echo "Building output2..." && cd output2 && RUSTC_WRAPPER=$(SCCACHE) $(CARGO) build 2>&1 | grep -E "error|^Compiling|^Finished" || true

clean_output2:
	@if [ ! -d "output2/.git" ]; then \
		$(MAKE) reinit_output2; \
	else \
		echo "output2 directory found, resetting and cleaning..."; \
		(cd output2 && git reset --hard HEAD && git clean -fdx); \
	fi

reinit_output2:
	rm -rf output2
	mkdir -p output2
	git init output2
	(cd output2 && touch .placeholder && git add .placeholder && git commit -m "Initial commit")

.PHONY: clean
clean:
	$(CARGO) clean
	rm -rf output2
	# git submodule deinit -f output2 # Related to submodule
	# git rm -f output2 # Related to submodule
	# rm -rf .git/modules/output2 # Related to submodule
