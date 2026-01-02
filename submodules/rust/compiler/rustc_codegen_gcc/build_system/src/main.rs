mkuse!{use std :: { env , process } ;}
mkmod!{abi_test, { 
                getname!(abi_test);
                getsrc!(abi_test);
                getpath!(abi_test);
                get_deps!(abi_test);
                get_crates!(abi_test);
                mkinclude!(abi_test);
                 
            }}
mkmod!{build, { 
                getname!(build);
                getsrc!(build);
                getpath!(build);
                get_deps!(build);
                get_crates!(build);
                mkinclude!(build);
                 
            }}
mkmod!{clean, { 
                getname!(clean);
                getsrc!(clean);
                getpath!(clean);
                get_deps!(clean);
                get_crates!(clean);
                mkinclude!(clean);
                 
            }}
mkmod!{clone_gcc, { 
                getname!(clone_gcc);
                getsrc!(clone_gcc);
                getpath!(clone_gcc);
                get_deps!(clone_gcc);
                get_crates!(clone_gcc);
                mkinclude!(clone_gcc);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                 
            }}
mkmod!{fmt, { 
                getname!(fmt);
                getsrc!(fmt);
                getpath!(fmt);
                get_deps!(fmt);
                get_crates!(fmt);
                mkinclude!(fmt);
                 
            }}
mkmod!{fuzz, { 
                getname!(fuzz);
                getsrc!(fuzz);
                getpath!(fuzz);
                get_deps!(fuzz);
                get_crates!(fuzz);
                mkinclude!(fuzz);
                 
            }}
mkmod!{info, { 
                getname!(info);
                getsrc!(info);
                getpath!(info);
                get_deps!(info);
                get_crates!(info);
                mkinclude!(info);
                 
            }}
mkmod!{prepare, { 
                getname!(prepare);
                getsrc!(prepare);
                getpath!(prepare);
                get_deps!(prepare);
                get_crates!(prepare);
                mkinclude!(prepare);
                 
            }}
mkmod!{rust_tools, { 
                getname!(rust_tools);
                getsrc!(rust_tools);
                getpath!(rust_tools);
                get_deps!(rust_tools);
                get_crates!(rust_tools);
                mkinclude!(rust_tools);
                 
            }}
mkmod!{rustc_info, { 
                getname!(rustc_info);
                getsrc!(rustc_info);
                getpath!(rustc_info);
                get_deps!(rustc_info);
                get_crates!(rustc_info);
                mkinclude!(rustc_info);
                 
            }}
mkmod!{test, { 
                getname!(test);
                getsrc!(test);
                getpath!(test);
                get_deps!(test);
                get_crates!(test);
                mkinclude!(test);
                 
            }}
mkmod!{utils, { 
                getname!(utils);
                getsrc!(utils);
                getpath!(utils);
                get_deps!(utils);
                get_crates!(utils);
                mkinclude!(utils);
                 
            }}
mkitem!{const BUILD_DIR : & str = "build" ;}
mkitem!{macro_rules ! arg_error { ($ ($ err : tt) *) => { { eprintln ! ($ ($ err) *) ; eprintln ! () ; usage () ; std :: process :: exit (1) ; } } ; }}

macro_rules! usage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function usage in module {}", module_path!());
    };
}

mkfn!{
    usage_introspect!();
    fn usage () { println ! ("\
rustc_codegen_gcc build system

Usage: build_system [command] [options]

Options:
        --help    : Displays this help message.

Commands:
        cargo     : Executes a cargo command.
        rustc     : Compiles the program using the GCC compiler.
        clean     : Cleans the build directory, removing all compiled files and artifacts.
        prepare   : Prepares the environment for building, including fetching dependencies and setting up configurations.
        build     : Compiles the project.
        test      : Runs tests for the project.
        info      : Displays information about the build environment and project configuration.
        clone-gcc : Clones the GCC compiler from a specified source.
        fmt       : Runs rustfmt
        fuzz      : Fuzzes `cg_gcc` using rustlantis
        abi-test   : Runs the abi-cafe test suite on the codegen, checking for ABI compatibility with LLVM") ; }
}
mkitem!{mkenum!{pub enum Command { Cargo , Clean , CloneGcc , Prepare , Build , Rustc , Test , Info , Fmt , Fuzz , AbiTest , }}}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { if env :: var ("RUST_BACKTRACE") . is_err () { unsafe { env :: set_var ("RUST_BACKTRACE" , "1") ; } } let command = match env :: args () . nth (1) . as_deref () { Some ("cargo") => Command :: Cargo , Some ("rustc") => Command :: Rustc , Some ("clean") => Command :: Clean , Some ("prepare") => Command :: Prepare , Some ("build") => Command :: Build , Some ("test") => Command :: Test , Some ("info") => Command :: Info , Some ("clone-gcc") => Command :: CloneGcc , Some ("abi-test") => Command :: AbiTest , Some ("fmt") => Command :: Fmt , Some ("fuzz") => Command :: Fuzz , Some ("--help") => { usage () ; process :: exit (0) ; } Some (flag) if flag . starts_with ('-') => arg_error ! ("Expected command found flag {}" , flag) , Some (command) => arg_error ! ("Unknown command {}" , command) , None => { usage () ; process :: exit (0) ; } } ; if let Err (e) = match command { Command :: Cargo => rust_tools :: run_cargo () , Command :: Rustc => rust_tools :: run_rustc () , Command :: Clean => clean :: run () , Command :: Prepare => prepare :: run () , Command :: Build => build :: run () , Command :: Test => test :: run () , Command :: Info => info :: run () , Command :: CloneGcc => clone_gcc :: run () , Command :: Fmt => fmt :: run () , Command :: Fuzz => fuzz :: run () , Command :: AbiTest => abi_test :: run () , } { eprintln ! ("Command failed to run: {e}") ; process :: exit (1) ; } }
}