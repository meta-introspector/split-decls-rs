SCCACHE := $(HOME)/.cargo/bin/sccache
# Use RUSTC_WRAPPER for sccache
CARGO := cargo

.PHONY: all build_core run_bootstrap build_output_module clean_output2

all: build_core run_bootstrap check_bootstrap_errors build_output_module

build_core:
	RUSTC_WRAPPER=$(SCCACHE) $(CARGO) build

run_bootstrap: clean_output2
	RUSTC_WRAPPER=$(SCCACHE) RUST_BACKTRACE=full $(CARGO) run --bin split-decls-rs -- bootstrap > temp_bootstrap.log 2>&1

check_bootstrap_errors:
	grep -E "error\[|error:" temp_bootstrap.log || true


build_output_module:
	cd output2 && RUSTC_WRAPPER=$(SCCACHE) $(CARGO) build

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
