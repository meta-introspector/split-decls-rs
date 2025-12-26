# Add these profile sections to your Cargo.toml

[profile.release]
debug = true  # Enable debug symbols for profiling

[profile.dev]
debug = true
opt-level = 1  # Some optimization for better profiling data
