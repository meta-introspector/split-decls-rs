// Macro definitions for wrapping Rust constructs with handlers

macro_rules! mkfn {
    ($introspect:expr; fn $name:ident() $body:block) => {
        fn $name() {
            $introspect;
            println!("🚀 MACRO INTERCEPTED: {}", stringify!($name));
            let result = (|| $body)();
            println!("🎯 MACRO COMPLETED: {}", stringify!($name));
            result
        }
    };
    ($introspect:expr; fn $name:ident($($param:ident: $ptype:ty),*) $body:block) => {
        fn $name($($param: $ptype),*) {
            $introspect;
            println!("🚀 MACRO INTERCEPTED: {}", stringify!($name));
            let result = (|| $body)();
            println!("🎯 MACRO COMPLETED: {}", stringify!($name));
            result
        }
    };
    ($introspect:expr; fn $name:ident($($param:ident: $ptype:ty),*) -> $ret:ty $body:block) => {
        fn $name($($param: $ptype),*) -> $ret {
            $introspect;
            println!("🚀 MACRO INTERCEPTED: {} -> {}", stringify!($name), stringify!($ret));
            let result: $ret = (|| $body)();
            println!("🎯 MACRO COMPLETED: {} -> {}", stringify!($name), stringify!($ret));
            result
        }
    };
}

macro_rules! mkitem {
    ($item:item) => { $item };
}

macro_rules! mkmod {
    (pub mod $name:ident { $($content:tt)* }) => {
        pub mod $name {
            $($content)*
        }
    };
    (mod $name:ident { $($content:tt)* }) => {
        mod $name {
            $($content)*
        }
    };
}

macro_rules! mkuse {
    ($use_stmt:item) => { $use_stmt };
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
