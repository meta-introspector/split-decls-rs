// Generated macro for run_rustfmt_recursively (function)
macro_rules! Depcrate_fmtrun_rustfmt_recursively {
() => {
// Module: crate::fmt
// Provides: {"run_rustfmt_recursively"}
// Dependencies: {}
fn run_rustfmt_recursively < P > (dir : P , check : bool) -> Result < () , String > where P : AsRef < Path > , { walk_dir (dir , & mut | dir | run_rustfmt_recursively (dir , check) , & mut | file_path | { if file_path . extension () . filter (| ext | ext == & OsStr :: new ("rs")) . is_some () { let rustfmt_cmd : & [& dyn AsRef < OsStr >] = if check { & [& "rustfmt" , & "--check" , & file_path] } else { & [& "rustfmt" , & file_path] } ; run_command_with_output (rustfmt_cmd , Some (Path :: new ("."))) } else { Ok (()) } } , true ,) }
};
}
