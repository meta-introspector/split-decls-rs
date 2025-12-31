# Genesis Makefile - Bootstrap the Self-Creating System from Nothing
.PHONY: genesis init_git init_nix init_cargo init_buildrs world

# THE ULTIMATE GENESIS - Create everything from nothing
genesis: init_git init_nix init_cargo init_buildrs
	@echo "🧬 GENESIS COMPLETE - Self-creating system born from void!"

# Step 1: Clean Git Repository
init_git:
	@echo "🌱 GENESIS: Initializing clean git repository..."
	git init
	git config user.name "Genesis System"
	git config user.email "genesis@split-decls.rs"
	@echo "✅ Clean git repository created"

# Step 2: Generate flake.nix from template
init_nix:
	@echo "❄️ GENESIS: Generating flake.nix..."
	cp templates/flake.nix ./flake.nix
	@echo "✅ flake.nix generated"

# Step 3: Generate Cargo.toml from template
init_cargo:
	@echo "📦 GENESIS: Generating Cargo.toml..."
	cp templates/Cargo.toml ./Cargo.toml
	@echo "✅ Cargo.toml generated"

# Step 4: Generate build.rs from template
init_buildrs:
	@echo "🏗️ GENESIS: Generating build.rs..."
	mkdir -p src
	cp templates/build.rs ./build.rs
	cp templates/main.rs ./src/main.rs
	cp templates/lib.rs ./src/lib.rs
	@echo "✅ Build system generated"

# Test the system
world: genesis
	@echo "🌍 TESTING WORLD EMERGENCE..."
	cargo build
	@echo "🌍 WORLD EMERGENCE COMPLETE!"
