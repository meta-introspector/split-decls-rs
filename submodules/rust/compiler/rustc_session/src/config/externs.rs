mkuse!{use std :: path :: PathBuf ;}
mkuse!{use rustc_errors :: { Diag , FatalAbort } ;}
mkuse!{use super :: UnstableOptions ;}
mkuse!{use crate :: EarlyDiagCtxt ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{mkstruct!{# [doc = " Represents the pieces of an `--extern` argument."] pub (crate) struct ExternOpt { pub (crate) crate_name : String , pub (crate) path : Option < PathBuf > , pub (crate) options : Option < String > , }}}

macro_rules! split_extern_opt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function split_extern_opt in module {}", module_path!());
    };
}

mkfn!{
    split_extern_opt_introspect!();
    # [doc = " Breaks out the major components of an `--extern` argument."] # [doc = ""] # [doc = " The options field will be a string containing comma-separated options that will need further"] # [doc = " parsing and processing."] pub (crate) fn split_extern_opt < 'a > (early_dcx : & 'a EarlyDiagCtxt , unstable_opts : & UnstableOptions , extern_opt : & str ,) -> Result < ExternOpt , Diag < 'a , FatalAbort > > { let (name , path) = match extern_opt . split_once ('=') { None => (extern_opt . to_string () , None) , Some ((name , path)) => (name . to_string () , Some (PathBuf :: from (path))) , } ; let (options , crate_name) = match name . split_once (':') { None => (None , name) , Some ((opts , crate_name)) => { if unstable_opts . namespaced_crates && crate_name . starts_with (':') { (None , name) } else { (Some (opts . to_string ()) , crate_name . to_string ()) } } } ; if ! valid_crate_name (& crate_name , unstable_opts) { let mut error = early_dcx . early_struct_fatal (format ! ("crate name `{crate_name}` passed to `--extern` is not a valid ASCII identifier")) ; let adjusted_name = crate_name . replace ('-' , "_") ; if is_ascii_ident (& adjusted_name) { # [allow (rustc :: diagnostic_outside_of_impl)] error . help (format ! ("consider replacing the dashes with underscores: `{adjusted_name}`")) ; } return Err (error) ; } Ok (ExternOpt { crate_name , path , options }) }
}

macro_rules! valid_crate_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function valid_crate_name in module {}", module_path!());
    };
}

mkfn!{
    valid_crate_name_introspect!();
    fn valid_crate_name (name : & str , unstable_opts : & UnstableOptions) -> bool { match name . split_once ("::") { Some ((a , b)) if unstable_opts . namespaced_crates => is_ascii_ident (a) && is_ascii_ident (b) , Some (_) => false , None => is_ascii_ident (name) , } }
}

macro_rules! is_ascii_ident_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_ascii_ident in module {}", module_path!());
    };
}

mkfn!{
    is_ascii_ident_introspect!();
    fn is_ascii_ident (string : & str) -> bool { let mut chars = string . chars () ; if let Some (start) = chars . next () && (start . is_ascii_alphabetic () || start == '_') { chars . all (| char | char . is_ascii_alphanumeric () || char == '_') } else { false } }
}