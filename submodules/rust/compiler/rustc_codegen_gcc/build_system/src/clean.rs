mkuse!{use std :: fs :: remove_dir_all ;}
mkuse!{use std :: path :: Path ;}
mkuse!{use crate :: utils :: { get_sysroot_dir , remove_file , run_command } ;}
mkitem!{mkenum!{# [derive (Default)] enum CleanArg { # [doc = " `clean all`"] All , # [doc = " `clean ui-tests`"] UiTests , # [doc = " `clean --help`"] # [default] Help , }}}
mkitem!{mkimpl!{impl CleanArg { fn new () -> Result < Self , String > { if let Some (arg) = std :: env :: args () . nth (2) { return match arg . as_str () { "all" => Ok (Self :: All) , "ui-tests" => Ok (Self :: UiTests) , "--help" => Ok (Self :: Help) , a => Err (format ! ("Unknown argument `{a}`")) , } ; } Ok (Self :: default ()) } }}}

macro_rules! usage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function usage in module {}", module_path!());
    };
}

mkfn!{
    usage_introspect!();
    fn usage () { println ! (r#"
`clean` command help:

    all                      : Clean all data
    ui-tests                 : Clean ui tests
    --help                   : Show this help
"#) }
}

macro_rules! clean_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clean_all in module {}", module_path!());
    };
}

mkfn!{
    clean_all_introspect!();
    fn clean_all () -> Result < () , String > { let build_sysroot = get_sysroot_dir () ; let dirs_to_remove = ["target" . into () , build_sysroot . join ("sysroot") , build_sysroot . join ("sysroot_src") , build_sysroot . join ("target") ,] ; for dir in dirs_to_remove { let _ = remove_dir_all (dir) ; } let dirs_to_remove = ["regex" , "rand" , "simple-raytracer"] ; for dir in dirs_to_remove { let _ = remove_dir_all (Path :: new (crate :: BUILD_DIR) . join (dir)) ; } let files_to_remove = [build_sysroot . join ("Cargo.lock") , "perf.data" . into () , "perf.data.old" . into ()] ; for file in files_to_remove { let _ = remove_file (& file) ; } println ! ("Successfully ran `clean all`") ; Ok (()) }
}

macro_rules! clean_ui_tests_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clean_ui_tests in module {}", module_path!());
    };
}

mkfn!{
    clean_ui_tests_introspect!();
    fn clean_ui_tests () -> Result < () , String > { let path = Path :: new (crate :: BUILD_DIR) . join ("rust/build/x86_64-unknown-linux-gnu/test/ui/") ; run_command (& [& "find" , & path , & "-name" , & "stamp" , & "-delete"] , None) ? ; Ok (()) }
}

macro_rules! run_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run in module {}", module_path!());
    };
}

mkfn!{
    run_introspect!();
    pub fn run () -> Result < () , String > { match CleanArg :: new () ? { CleanArg :: All => clean_all () ? , CleanArg :: UiTests => clean_ui_tests () ? , CleanArg :: Help => usage () , } Ok (()) }
}