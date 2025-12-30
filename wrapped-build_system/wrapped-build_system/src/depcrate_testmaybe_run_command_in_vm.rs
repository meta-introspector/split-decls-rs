// Generated macro for maybe_run_command_in_vm (function)
macro_rules! Depcrate_testmaybe_run_command_in_vm {
() => {
// Module: crate::test
// Provides: {"maybe_run_command_in_vm"}
// Dependencies: {}
fn maybe_run_command_in_vm (command : & [& dyn AsRef < OsStr >] , env : & Env , args : & TestArg ,) -> Result < () , String > { if ! args . config_info . run_in_vm { run_command_with_output_and_env (command , None , Some (env)) ? ; return Ok (()) ; } let vm_parent_dir = match env . get ("CG_GCC_VM_DIR") { Some (dir) if ! dir . is_empty () => PathBuf :: from (dir . clone ()) , _ => std :: env :: current_dir () . unwrap () , } ; let vm_dir = "vm" ; let exe_to_run = command . first () . unwrap () ; let exe = Path :: new (& exe_to_run) ; let exe_filename = exe . file_name () . unwrap () ; let vm_home_dir = vm_parent_dir . join (vm_dir) . join ("home") ; let vm_exe_path = vm_home_dir . join (exe_filename) ; let inside_vm_exe_path = Path :: new ("/home") . join (exe_filename) ; let sudo_command : & [& dyn AsRef < OsStr >] = & [& "sudo" , & "cp" , & exe , & vm_exe_path] ; run_command_with_env (sudo_command , None , Some (env)) ? ; let mut vm_command : Vec < & dyn AsRef < OsStr > > = vec ! [& "sudo" , & "chroot" , & vm_dir , & "qemu-m68k-static" , & inside_vm_exe_path] ; vm_command . extend_from_slice (command) ; run_command_with_output_and_env (& vm_command , Some (& vm_parent_dir) , Some (env)) ? ; Ok (()) }
};
}
