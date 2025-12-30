// Generated macro for generate_checked_macros_impl (function)
macro_rules! Depcrate_template_checkergenerate_checked_macros_impl {
() => {
// Module: crate::template_checker
// Provides: {"generate_checked_macros_impl"}
// Dependencies: {}
# [decl (fn , name = "generate_checked_macros_impl" , vis = "pub" , hash = "08b7b728")] pub fn generate_checked_macros_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let _config = input_str . value () ; quote ! { { println ! ("cargo:warning=🔧 Generating syntax-checked common macros") ; let common_macros_template = r#"
// Auto-generated common subexpression macros - SYNTAX CHECKED

/// Macro for common quote pattern: quote! { ... }.into()
macro_rules! quote_into {
    ($($tokens:tt)*) => {
        quote! { $($tokens)* }.into()
    };
}

/// Macro for common warning pattern
macro_rules! build_warning {
    ($msg:expr) => {
        println!("cargo:warning={}", $msg)
    };
    ($fmt:expr, $($args:expr),*) => {
        println!("cargo:warning={}", format!($fmt, $($args),*))
    };
}

/// Macro for common input parsing
macro_rules! parse_string_input {
    ($input:expr) => {
        parse_macro_input!($input as LitStr).value()
    };
}

/// Macro for common format pattern
macro_rules! format_template {
    ($template:expr, $($args:expr),*) => {
        format!($template, $($args),*)
    };
}

/// Macro for common vector creation
macro_rules! items_vec {
    ($($item:expr),*) => {
        vec![$($item),*]
    };
}

/// Macro for common error handling
macro_rules! unwrap_or_default {
    ($expr:expr, $default:expr) => {
        $expr.unwrap_or_else(|_| $default)
    };
}

/// Macro for common file writing
macro_rules! write_file {
    ($path:expr, $content:expr) => {
        std::fs::write($path, $content).ok()
    };
}

/// Macro for common proc_macro function pattern
macro_rules! proc_macro_fn {
    ($name:ident, $impl_fn:path) => {
        #[proc_macro]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $impl_fn(input)
        }
    };
}
            "# ; let checked_template = crate :: checktemplate ! (common_macros_template) ; checked_template } } . into () }
};
}
