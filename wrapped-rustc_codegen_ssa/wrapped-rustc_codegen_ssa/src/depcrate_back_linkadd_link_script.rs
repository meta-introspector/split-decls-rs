// Generated macro for add_link_script (function)
macro_rules! Depcrate_back_linkadd_link_script {
() => {
// Module: crate::back::link
// Provides: {"add_link_script"}
// Dependencies: {}
# [doc = " Add a link script embedded in the target, if applicable."] fn add_link_script (cmd : & mut dyn Linker , sess : & Session , tmpdir : & Path , crate_type : CrateType) { match (crate_type , & sess . target . link_script) { (CrateType :: Cdylib | CrateType :: Executable , Some (script)) => { if ! sess . target . linker_flavor . is_gnu () { sess . dcx () . emit_fatal (errors :: LinkScriptUnavailable) ; } let file_name = ["rustc" , & sess . target . llvm_target , "linkfile.ld"] . join ("-") ; let path = tmpdir . join (file_name) ; if let Err (error) = fs :: write (& path , script . as_ref ()) { sess . dcx () . emit_fatal (errors :: LinkScriptWriteFailure { path , error }) ; } cmd . link_arg ("--script") . link_arg (path) ; } _ => { } } }
};
}
