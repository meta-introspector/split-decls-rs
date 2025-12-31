{
  description = "Step3 Genesis - Advanced Compiler Integration";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  
  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "genesis-step3";
        version = "0.3.0";
        src = ./.;
        cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
        
        buildInputs = with pkgs; [ llvm ];
        
        buildPhase = ''
          echo "🌟 Building Step3 Genesis with compiler integration..."
          cargo build --release
          mkdir -p $out/bin
          cp target/release/* $out/bin/ || true
        '';
      };
      
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
          cargo
          git
          nix
          llvm
          clang
        ];
        
        shellHook = ''
          echo "🌟 Step3 Genesis Environment Ready"
          echo "Enhanced with: Full compiler toolchain + LLVM + Advanced macros"
          echo "Run: cargo run --bin step3_bootstrap"
        '';
      };
    };
}
