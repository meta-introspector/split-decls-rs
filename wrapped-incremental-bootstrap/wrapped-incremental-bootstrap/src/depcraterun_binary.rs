// Generated macro for run_binary (function)
macro_rules! Depcraterun_binary {
() => {
// Module: crate
// Provides: {"run_binary"}
// Dependencies: {}
fn run_binary (bin_name : & str , args : Vec < String >) -> Result < () , Box < dyn std :: error :: Error > > { println ! ("🚀 Running binary: {}" , bin_name) ; let mut cmd = match bin_name { "simple_split" => { let mut c = Command :: new ("cargo") ; c . args (& ["run" , "--manifest-path" , "../simple-split/Cargo.toml"]) ; c } , "split-decls-rs" => { let mut c = Command :: new ("cargo") ; c . args (& ["run" , "--manifest-path" , "../Cargo.toml"]) ; c } , "bootstrap" => { let mut c = Command :: new ("cargo") ; c . args (& ["run" , "--manifest-path" , "../Cargo.toml" , "--" , "bootstrap"]) ; c } , "incremental-builder" => { let mut c = Command :: new ("cargo") ; c . args (& ["run" , "--manifest-path" , "../incremental-builder/Cargo.toml"]) ; c } , "topo-sorter" => { let mut c = Command :: new ("cargo") ; c . args (& ["run" , "--manifest-path" , "../topo-sorter/Cargo.toml"]) ; c } , "index-generator" => { let mut c = Command :: new ("cargo") ; c . args (& ["run" , "--manifest-path" , "../index-generator/Cargo.toml"]) ; c } , _ => { println ! ("❌ Unknown binary: {}" , bin_name) ; println ! ("Run 'list-bins' to see available binaries") ; return Ok (()) ; } } ; if ! args . is_empty () { cmd . args (& args) ; } let status = cmd . status () ? ; if status . success () { println ! ("✅ Binary {} completed successfully" , bin_name) ; } else { println ! ("❌ Binary {} failed with exit code: {:?}" , bin_name , status . code ()) ; } Ok (()) }
};
}
