// SRC: ../rust/compiler/rustc_codegen_cranelift/scripts/cargo-clif.rs
use std::env;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::Command;

// Inlined shared_utils.rs
// This file is used by both the build system as well as cargo-clif.rs

// Adapted from https://github.com/rust-lang/cargo/blob/6dc1deaddf62c7748c9097c7ea88e9ec77ff1a1a/src/cargo/core/compiler/build_context/target_info.rs#L750-L77
pub(crate) fn rustflags_from_env(kind: &str) -> Vec<String> {
    // First try CARGO_ENCODED_RUSTFLAGS from the environment.
    // Prefer this over RUSTFLAGS since it's less prone to encoding errors.
    if let Ok(a) = std::env::var(format!("CARGO_ENCODED_{}", kind)) {
        if a.is_empty() {
            return Vec::new();
        }
        return a.split('\x1f').map(str::to_string).collect();
    }

    // Then try RUSTFLAGS from the environment
    if let Ok(a) = std::env::var(kind) {
        let args = a.split(' ').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string);
        return args.collect();
    }

    // No rustflags to be collected from the environment
    Vec::new()
}

pub(crate) fn rustflags_to_cmd_env(cmd: &mut std::process::Command, kind: &str, flags: &[String]) {
    cmd.env(format!("CARGO_ENCODED_{}", kind), flags.join("\x1f"));
}


fn main() {
    let current_exe = env::current_exe().unwrap();
    let mut sysroot = current_exe.parent().unwrap();
    if sysroot.file_name().unwrap().to_str().unwrap() == "bin" {
        sysroot = sysroot.parent().unwrap();
    }

    let mut rustflags = vec!["-Cpanic=abort".to_owned(), "-Zpanic-abort-tests".to_owned()];
    if let Some(name) = option_env!("BUILTIN_BACKEND") {
        rustflags.push(format!("-Zcodegen-backend={name}"));
    } else {
        let dylib = sysroot.join("lib").join(
            env::consts::DLL_PREFIX.to_string()
                + "rustc_codegen_cranelift"
                + env::consts::DLL_SUFFIX,
        );
        rustflags.push(format!("-Zcodegen-backend={}", dylib.to_str().unwrap()));
    }
    rustflags.push("--sysroot".to_owned());
    rustflags.push(sysroot.to_str().unwrap().to_owned());

    let cargo = if let Some(cargo) = option_env!("CARGO") {
        cargo
    } else {
        // Ensure that the right toolchain is used
        env::set_var("RUSTUP_TOOLCHAIN", option_env!("TOOLCHAIN_NAME").expect("TOOLCHAIN_NAME"));
        "cargo"
    };

    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.get(0).map(|arg| &**arg) == Some("clif") {
        // Avoid infinite recursion when invoking `cargo-clif` as cargo subcommand using
        // `cargo clif`.
        args.remove(0);
    }

    let args: Vec<_> = match args.get(0).map(|arg| &**arg) {
        Some("jit") => {
            rustflags.push("-Cprefer-dynamic".to_owned());
            args.remove(0);
            IntoIterator::into_iter(["rustc".to_string()])
                .chain(args)
                .chain([
                    "--".to_string(),
                    "-Zunstable-options".to_string(),
                    "-Cllvm-args=jit-mode".to_string(),
                ])
                .collect()
        }
        _ => args,
    };

    let mut cmd = Command::new(cargo);
    cmd.args(args);
    rustflags_to_cmd_env(
        &mut cmd,
        "RUSTFLAGS",
        &rustflags_from_env("RUSTFLAGS")
            .into_iter()
            .chain(rustflags.iter().map(|flag| flag.clone()))
            .collect::<Vec<_>>(),
    );
    rustflags_to_cmd_env(
        &mut cmd,
        "RUSTDOCFLAGS",
        &rustflags_from_env("RUSTDOCFLAGS")
            .into_iter()
            .chain(rustflags.iter().map(|flag| flag.clone()))
            .collect::<Vec<_>>(),
    );

    #[cfg(unix)]
    panic!("Failed to spawn cargo: {}", cmd.exec());

    #[cfg(not(unix))]
    std::process::exit(cmd.spawn().unwrap().wait().unwrap().code().unwrap_or(1));
}