// macro2clippy! - Convert macros to clippy commands for codebase transformation

#[macro_export]
macro_rules! macro2clippy {
    // Convert a macro to a clippy fix command
    ($macro_name:ident, $pattern:expr, $replacement:expr) => {
        pub fn generate_clippy_command() -> String {
            format!(
                "cargo clippy --fix --allow-dirty --allow-staged -- -A clippy::all -W clippy::manual_map && \
                 find . -name '*.rs' -exec sed -i 's/{}/{}/g' {{}} \\;",
                $pattern,
                $replacement
            )
        }
    };
    
    // Apply macro transformation across entire codebase
    ($macro_name:ident) => {
        pub fn apply_macro_transformation() -> std::process::Command {
            let mut cmd = std::process::#[syscall="exec"]
    Command::new("sh");
            cmd.arg("-c")
               .arg(format!(
                   "find . -name '*.rs' -exec grep -l '{}' {{}} \\; | \
                    xargs -I {{}} cargo expand --bin {{}} | \
                    cargo clippy --fix --allow-dirty",
                   stringify!($macro_name)
               ));
            cmd
        }
    };
}

// Usage examples:

// Convert track_file_read! to clippy-fixable pattern
macro2clippy!(track_file_read, 
    r"track_file_read!\(([^)]+)\)", 
    r"#[syscall="read"]
    std::fs::read_to_string(\1).map_err(|e| eprintln!(\"MANIFEST_READ: {}\", \1))"
);

// Convert track_file_write! to clippy-fixable pattern  
macro2clippy!(track_file_write,
    r"track_file_write!\(([^,]+),\s*([^)]+)\)",
    r"#[syscall="write"]
    std::fs::write(\1, \2).map_err(|e| eprintln!(\"MANIFEST_WRITE: {}\", \1))"
);

// Apply report_manifest! across all binaries
macro2clippy!(report_manifest);

// Generate upgrade command for entire codebase
pub fn upgrade_codebase_with_manifest_tracking() -> String {
    format!(
        "#!/bin/bash
        # Auto-generated clippy command to add manifest tracking
        
        # Step 1: Add manifest macros to all binaries
        find src/bin -name '*.rs' -exec sed -i '1i use crate::manifest_macros::*;' {{}} \\;
        
        # Step 2: Replace #[syscall="read"]
    std::fs::read_to_string with track_file_read!
        find . -name '*.rs' -exec sed -i 's/#[syscall="read"]
    std::fs::read_to_string(/track_file_read!(/g' {{}} \\;
        
        # Step 3: Replace std::fs::write with track_file_write!
        find . -name '*.rs' -exec sed -i 's/#[syscall="write"]
    std::fs::write(/track_file_write!(/g' {{}} \\;
        
        # Step 4: Add manifest reporting to each binary
        find src/bin -name '*.rs' -exec bash -c 'echo \"report_manifest!(\\\"$(basename \"$1\" .rs)\\\");\" >> \"$1\"' _ {{}} \\;
        
        # Step 5: Run clippy to fix any issues
        cargo clippy --fix --allow-dirty --allow-staged
        
        # Step 6: Generate bootstrap manifest
        cargo run --bin generate_bootstrap_manifest
        "
    )
}

// Specific macro for our use case
#[macro_export]
macro_rules! upgrade_to_manifest_tracking {
    () => {
        // Generate the upgrade script
        #[syscall="write"]
    std::fs::write("upgrade_manifest.sh", upgrade_codebase_with_manifest_tracking())?;
        std::process::#[syscall="exec"]
    Command::new("chmod")
            .arg("+x")
            .arg("upgrade_manifest.sh")
            .status()?;
        
        println!("Generated upgrade_manifest.sh - run it to add manifest tracking to all code");
    };
}
