use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::LazyLock;

static USE_MATRIX: LazyLock<Mutex<HashMap<String, Vec<String>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn get_use_matrix() -> HashMap<String, Vec<String>> {
    USE_MATRIX.lock().unwrap().clone()
}

// Macro definitions for wrapping Rust constructs with handlers

// Replace problematic print statements with emit_message
macro_rules! emit_message {
    ($($arg:tt)*) => {
        {
            use std::fs::OpenOptions;
            use std::io::Write;
            let message = format!($($arg)*);
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("macro_report.txt") {
                let _ = writeln!(file, "{}", message);
            }
        }
    };
}

macro_rules! mkfn {
    // MARKER: pub_trait_bounds_generic
    ($introspect:expr; $(#[$attr:meta])* pub fn $name:ident < F : FnOnce ( ) -> R , R > ($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* pub fn $name < F : FnOnce ( ) -> R , R > ($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: pub_trait_bounds_generic - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: pub_trait_bounds_generic - {}", stringify!($name));
            result
        }
    };
    // MARKER: trait_bounds_generic
    ($introspect:expr; $(#[$attr:meta])* fn $name:ident < F : FnOnce ( ) -> R , R > ($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* fn $name < F : FnOnce ( ) -> R , R > ($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: trait_bounds_generic - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: trait_bounds_generic - {}", stringify!($name));
            result
        }
    };
    // MARKER: catch_fatal_errors_specific
    ($introspect:expr; $(#[$attr:meta])* fn catch_fatal_errors < F : FnOnce ( ) -> R , R > ($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* fn catch_fatal_errors < F : FnOnce ( ) -> R , R > ($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: catch_fatal_errors_specific");
            let result = (|| $body)();
            emit_message!("🎯 MARKER: catch_fatal_errors_specific");
            result
        }
    };
    // MARKER: parse_crate_attrs_specific
    ($introspect:expr; $(#[$attr:meta])* fn parse_crate_attrs < $lifetime:lifetime > ($($param:tt)*) -> PResult < $lifetime2:lifetime , ast :: AttrVec > $body:block) => {
        $(#[$attr])* fn parse_crate_attrs < $lifetime > ($($param)*) -> PResult < $lifetime2 , ast :: AttrVec > {
            $introspect;
            emit_message!("🚀 MARKER: parse_crate_attrs_specific");
            let result = (|| $body)();
            emit_message!("🎯 MARKER: parse_crate_attrs_specific");
            result
        }
    };
    // MARKER: init_logger_specific
    ($introspect:expr; $(#[$attr:meta])* fn init_logger_with_additional_layer < F , T > ($($param:tt)*) where F : FnOnce ( ) -> T , T : rustc_log :: BuildSubscriberRet , $body:block) => {
        $(#[$attr])* fn init_logger_with_additional_layer < F , T > ($($param)*) where F : FnOnce ( ) -> T , T : rustc_log :: BuildSubscriberRet , {
            $introspect;
            emit_message!("🚀 MARKER: init_logger_specific");
            let result = (|| $body)();
            emit_message!("🎯 MARKER: init_logger_specific");
            result
        }
    };
    ($introspect:expr; $(#[$attr:meta])* fn $name:ident < F , T > ($($param:tt)*) $(-> $ret:ty)? where F : FnOnce ( $($fnonce_args:tt)* ) $($where_rest:tt)* $body:block) => {
        $(#[$attr])* fn $name < F , T > ($($param)*) $(-> $ret)? where F : FnOnce ( $($fnonce_args)* ) $($where_rest)* {
            $introspect;
            emit_message!("🚀 MARKER: two_generics_where_fnonce - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: two_generics_where_fnonce - {}", stringify!($name));
            result
        }
    };
    ($introspect:expr; $(#[$attr:meta])* fn $name:ident < $gen1:ident , $gen2:ident > ($($param:tt)*) $(-> $ret:ty)? where $($where_clause:tt)* $body:block) => {
        $(#[$attr])* fn $name < $gen1 , $gen2 > ($($param)*) $(-> $ret)? where $($where_clause)* {
            $introspect;
            emit_message!("🚀 MARKER: two_generics_where - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: two_generics_where - {}", stringify!($name));
            result
        }
    };
    // MARKER: two_generics
    ($introspect:expr; $(#[$attr:meta])* fn $name:ident < $gen1:ident , $gen2:ident > ($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* fn $name < $gen1 , $gen2 > ($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: two_generics - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: two_generics - {}", stringify!($name));
            result
        }
    };
    ($introspect:expr; $(#[$attr:meta])* pub fn $name:ident < $gen1:ident , $gen2:ident > ($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* pub fn $name < $gen1 , $gen2 > ($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: pub_two_generics - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: pub_two_generics - {}", stringify!($name));
            result
        }
    };
    ($introspect:expr; $(#[$attr:meta])* pub ($vis:ident) fn $name:ident($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* pub ($vis) fn $name($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: pub_vis - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: pub_vis - {}", stringify!($name));
            result
        }
    };
    // MARKER: generic_single
    ($introspect:expr; $(#[$attr:meta])* fn $name:ident < $gen:ident > ($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* fn $name < $gen > ($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: generic_single - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: generic_single - {}", stringify!($name));
            result
        }
    };
    ($introspect:expr; $(#[$attr:meta])* pub fn $name:ident < $gen:ident > ($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* pub fn $name < $gen > ($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: pub_generic_single - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: pub_generic_single - {}", stringify!($name));
            result
        }
    };
    // MARKER: lifetime
    ($introspect:expr; $(#[$attr:meta])* fn $name:ident < $lifetime:lifetime > ($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* fn $name < $lifetime > ($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: lifetime - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: lifetime - {}", stringify!($name));
            result
        }
    };
    ($introspect:expr; $(#[$attr:meta])* pub fn $name:ident < $lifetime:lifetime > ($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* pub fn $name < $lifetime > ($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: pub_lifetime - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: pub_lifetime - {}", stringify!($name));
            result
        }
    };
    // MARKER: non_generic
    ($introspect:expr; $(#[$attr:meta])* pub fn $name:ident($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* pub fn $name($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: pub_non_generic - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: pub_non_generic - {}", stringify!($name));
            result
        }
    };
    ($introspect:expr; $(#[$attr:meta])* fn $name:ident($($param:tt)*) $(-> $ret:ty)? $body:block) => {
        $(#[$attr])* fn $name($($param)*) $(-> $ret)? {
            $introspect;
            emit_message!("🚀 MARKER: non_generic - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: non_generic - {}", stringify!($name));
            result
        }
    };
    ($introspect:expr; fn $name:ident() $body:block) => {
        fn $name() {
            $introspect;
            emit_message!("🚀 MARKER: simple - {}", stringify!($name));
            let result = (|| $body)();
            emit_message!("🎯 MARKER: simple - {}", stringify!($name));
            result
        }
    };
    // MARKER: catch_all - matches anything not caught above
    ($introspect:expr; $($anything:tt)*) => {
        $($anything)*
    };
}

// Define missing macros
macro_rules! safe_println {
    ($($arg:tt)*) => {
        ()
    };
}

macro_rules! safe_print {
    ($($arg:tt)*) => {
        ()
    };
}

#[macro_export]
macro_rules! include_rust_compiler {
    ($crate_name:literal, $subpath:literal, $file:literal) => {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"), 
            "/processed_submodules_rust_compiler_rustc_", 
            $crate_name, 
            "_", 
            $subpath, 
            "_", 
            $file, 
            ".rs"
        ));
    };
    ($crate_name:literal, $file:literal) => {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"), 
            "/processed_submodules_rust_compiler_rustc_", 
            $crate_name, 
            "_src_", 
            $file, 
            ".rs"
        ));
    };
    ($crate_name:literal) => {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"), 
            "/processed_submodules_rust_compiler_rustc_", 
            $crate_name, 
            "_src_lib.rs"
        ));
    };
}

#[macro_export]
macro_rules! include_rust_library {
    ($lib_name:literal, $subpath:literal, $file:literal) => {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"), 
            "/processed_submodules_rust_library_", 
            $lib_name, 
            "_", 
            $subpath, 
            "_", 
            $file, 
            ".rs"
        ));
    };
    ($lib_name:literal, $file:literal) => {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"), 
            "/processed_submodules_rust_library_", 
            $lib_name, 
            "_src_", 
            $file, 
            ".rs"
        ));
    };
    ($lib_name:literal) => {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"), 
            "/processed_submodules_rust_library_", 
            $lib_name, 
            "_src_lib.rs"
        ));
    };
}

#[macro_export]
macro_rules! include_processed {
    ($path:literal) => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/processed_", $path, ".rs"));
    };
}

macro_rules! mkinclude {
    ($path:ident) => {
        // Skip include! calls with identifiers - they're problematic
    };
    ($path:literal) => {
        include!($path)
    };
}

macro_rules! mkitem {
    // Handle include! with identifier - direct pattern
    (include ! ($path:ident) ;) => {
        // Skip include! calls that don't have string literals
    };
    // Handle macro calls with string literals
    ($macro_name:ident :: $macro_sub:ident ! { $string_lit:literal }) => {
        $macro_name :: $macro_sub ! { $string_lit }
    };
    ($macro_name:ident :: $macro_sub:ident ! { $($args:tt)* }) => {
        $macro_name :: $macro_sub ! { $($args)* }
    };
    ($macro_name:ident ! { $($args:tt)* }) => {
        $macro_name ! { $($args)* }
    };
    ($item:item) => { $item };
}

#[macro_export]
macro_rules! mkmod {
    // Handle introspection pattern: mkmod!{name, { content }}
    ($name:ident, { $($content:tt)* }) => {
        emit_message!("MOD|{}|{}", module_path!(), stringify!($name));
        mod $name {
            const MODULE_NAME: &str = stringify!($name);
            $($content)*
        }
    };
    // Handle standard patterns
    (pub mod $name:ident { $($content:tt)* }) => {
        emit_message!("MOD|{}|{}", module_path!(), stringify!($name));
        pub mod $name {
            const MODULE_NAME: &str = stringify!($name);
            $($content)*
        }
    };
    (mod $name:ident { $($content:tt)* }) => {
        emit_message!("MOD|{}|{}", module_path!(), stringify!($name));
        mod $name {
            const MODULE_NAME: &str = stringify!($name);
            $($content)*
        }
    };
}

#[macro_export]
macro_rules! mkuse {
    ($use_stmt:item) => { 
        emit_message!("USE|{}|{}", module_path!(), stringify!($use_stmt));
        $use_stmt
    };
}

macro_rules! mkstruct {
    ($struct_def:item) => { $struct_def };
}

macro_rules! mkenum {
    ($enum_def:item) => { $enum_def };
}

macro_rules! mktrait {
    ($trait_def:item) => { $trait_def };
}

macro_rules! mkimpl {
    ($impl_def:item) => { $impl_def };
}

// Introspection macros
macro_rules! getname {
    ($name:ident) => {
        stringify!($name)
    };
}

macro_rules! getsrc {
    ($name:ident) => {
        "processed file"
    };
}

macro_rules! getpath {
    ($name:ident) => {
        "processed_path"
    };
}

macro_rules! get_deps {
    ($name:ident) => {
        vec![]
    };
}

pub mod rustc_complete {
    pub mod emitter {
        pub fn stderr_destination() {}
    }
    pub mod registry {
        pub struct Registry;
    }
    pub mod translation {
        pub struct Translator;
    }
    pub struct ColorConfig;
    pub struct DiagCtxt;
    pub struct ErrCode;
    pub struct FatalError;
    pub struct PResult<T>(pub T);
    pub mod markdown {}
    pub mod config {
        pub struct CG_OPTIONS;
        pub struct CrateType;
        pub struct ErrorOutputType;
        pub struct Input;
        pub struct OptionDesc;
        pub struct OutFileName;
        pub struct OutputType;
        pub struct Sysroot;
        pub struct UnstableOptions;
        pub struct Z_OPTIONS;
        pub fn nightly_options() {}
        pub fn parse_target_triple() {}
    }
    pub mod getopts {
        pub struct Matches;
    }
    pub mod lint {
        pub struct Lint;
        pub struct LintId;
    }
    pub mod output {
        pub struct CRATE_TYPES;
        pub fn collect_crate_types() {}
        pub fn invalid_output_for_target() {}
    }
    pub struct EarlyDiagCtxt;
    pub struct Session;
    pub struct FileName;
    pub mod def_id {
        pub struct LOCAL_CRATE;
    }
    pub mod ty {
        pub struct TyCtxt<T>(pub T);
    }
}

// Removed duplicate rustc_feature module - already defined in main file

pub mod session_diagnostics {
    pub struct CantEmitMIR;
    pub struct RLinkEmptyVersionNumber;
    pub struct RLinkEncodingVersionMismatch;
    pub struct RLinkRustcVersionMismatch;
    pub struct RLinkWrongFileType;
    pub struct RlinkCorruptFile;
    pub struct RlinkNotAFile;
    pub struct RlinkUnableToRead;
    pub struct UnstableFeatureUsage;
}

macro_rules! do_not_use_print {
    ($($t:tt)*) => {
        compile_error!("Don't use print")
    };
}

macro_rules! do_not_use_safe_print {
    ($($t:tt)*) => {
        compile_error!("Don't use safe_print")
    };
}

macro_rules! mktrait {
    ($trait_def:item) => { $trait_def };
}

macro_rules! mkimpl {
    ($impl_def:item) => { $impl_def };
}

macro_rules! getname {
    ($name:ident) => {
        pub fn get_module_name() -> &'static str { stringify!($name) }
    };
}

macro_rules! getsrc {
    ($name:ident) => {
        pub fn get_source_info() -> &'static str { concat!("Module: ", stringify!($name)) }
    };
}

macro_rules! getpath {
    ($name:ident) => {
        pub fn get_module_path() -> &'static str { module_path!() }
    };
}

macro_rules! get_deps {
    ($name:ident) => {
        pub fn get_dependencies() -> &'static [&'static str] {
            &[] // TODO: Extract from AST analysis
        }
    };
}

macro_rules! get_crates {
    ($name:ident) => {
        pub fn get_required_crates() -> &'static [&'static str] {
            &[] // TODO: Extract from use statements
        }
    };
}

macro_rules! forall_crates {
    ($($crate_name:ident),*) => {
        $(extern crate $crate_name;)*
    };
}

macro_rules! emit_extern {
    ($crate_name:ident) => {
        extern crate $crate_name;
    };
}

macro_rules! get_externs {
    ($crate_name:ident) => {
        stringify!($crate_name)
    };
}
