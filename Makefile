SCCACHE := $(HOME)/.cargo/bin/sccache
RUSTC := $(SCCACHE) rustc
CARGO := $(SCCACHE) cargo

.PHONY: all build_core run_bootstrap build_output_module clean_output2

all: build_core run_bootstrap build_output_module

build_core:
	$(CARGO) build

run_bootstrap: clean_output2
	RUST_BACKTRACE=full $(CARGO) run --bin split-decls-rs -- bootstrap

build_output_module:
	cd output2 && $(CARGO) build

clean_output2:
	rm -rf output2
	git submodule add file://$(CURDIR)/output2 output2 || true
	cd output2 && git reset --hard HEAD || true # Ensure the submodule is clean before re-adding

.PHONY: clean
clean:
	$(CARGO) clean
	rm -rf output2
	git submodule deinit -f output2
	git rm -f output2
	rm -rf .git/modules/output2
