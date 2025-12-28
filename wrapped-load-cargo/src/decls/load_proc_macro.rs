macro_rules! load_proc_macro {
    () => {
        # [doc = " Load the proc-macros for the given lib path, disabling all expanders whose names are in `ignored_macros`."] pub fn load_proc_macro (server : & ProcMacroClient , path : & AbsPath , ignored_macros : & [Box < str >] ,) -> ProcMacroLoadResult { let res : Result < Vec < _ > , _ > = (| | { let dylib = MacroDylib :: new (path . to_path_buf ()) ; let vec = server . load_dylib (dylib) . map_err (| e | { ProcMacroLoadingError :: ProcMacroSrvError (format ! ("{e}") . into_boxed_str ()) }) ? ; if vec . is_empty () { return Err (ProcMacroLoadingError :: NoProcMacros) ; } Ok (vec . into_iter () . map (| expander | expander_to_proc_macro (expander , ignored_macros)) . collect ()) }) () ; match res { Ok (proc_macros) => { tracing :: info ! ("Loaded proc-macros for {path}: {:?}" , proc_macros . iter () . map (| it | it . name . clone ()) . collect ::< Vec < _ >> ()) ; Ok (proc_macros) } Err (e) => { tracing :: warn ! ("proc-macro loading for {path} failed: {e}") ; Err (e) } } }
    };
}

load_proc_macro!()