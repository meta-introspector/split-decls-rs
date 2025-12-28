macro_rules! mkbuildrs_with_macros_impl {
    () => {
        # [decl (fn , name = "mkbuildrs_with_macros_impl" , vis = "pub" , hash = "152b1c84")] pub fn mkbuildrs_with_macros_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let config = input_str . value () ; quote ! { { println ! ("cargo:warning=🔧 mkbuildrs with common macros: {}" , # config) ; let build_rs_with_macros = r#"
// Enhanced build.rs with common macro utilities
use std::process::Command;

macro_rules! build_warning {
    ($msg:expr) => {
        println!("cargo:warning={}", $msg)
    };
}

macro_rules! unwrap_or_default {
    ($expr:expr, $default:expr) => {
        $expr.unwrap_or_else(|_| $default)
    };
}

fn main() {
    build_warning!("🔧 Enhanced build starting...");
    
    let nix_available = unwrap_or_default!(
        Command::new("which").arg("nix").output().map(|o| o.status.success()),
        false
    );
    
    if nix_available {
        build_warning!("✅ Nix available");
    } else {
        build_warning!("⚠️ Nix not available");
    }
    
    build_warning!("🔧 Enhanced build complete");
}
            "# ; build_rs_with_macros . to_string () } } . into () }
    };
}

mkbuildrs_with_macros_impl!()