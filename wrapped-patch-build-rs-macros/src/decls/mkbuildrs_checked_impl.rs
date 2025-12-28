macro_rules! mkbuildrs_checked_impl {
    () => {
        # [decl (fn , name = "mkbuildrs_checked_impl" , vis = "pub" , hash = "315bf7c5")] pub fn mkbuildrs_checked_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let config = input_str . value () ; let build_rs_template = r#"
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
    "# ; let parse_result = parse_str :: < syn :: File > (build_rs_template) ; match parse_result { Ok (_) => { quote ! { { println ! ("cargo:warning=🔧 mkbuildrs with syntax-checked macros: {}" , # config) ; println ! ("cargo:warning=✅ Build template syntax validated") ; # build_rs_template . to_string () } } . into () } Err (e) => { let error_msg = format ! ("❌ Build template syntax error: {}" , e) ; quote ! { { println ! ("cargo:warning={}" , # error_msg) ; compile_error ! (# error_msg) ; } } . into () } } }
    };
}

mkbuildrs_checked_impl!();