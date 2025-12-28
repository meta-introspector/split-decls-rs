macro_rules! RustcToolTrait {
    () => {
        # [async_trait] pub trait RustcToolTrait : Send + Sync { # [doc = " Compiles a Rust project or file."] async fn compile (& self , input_path : & PathBuf , output_path : & PathBuf) -> Result < () > ; # [doc = " Checks a Rust project or file for errors without producing an executable."] async fn check (& self , input_path : & PathBuf) -> Result < () > ; # [doc = " Returns the version of the rustc tool."] async fn version (& self) -> Result < String > ; # [doc = " Runs a custom rustc command with provided arguments."] async fn run_command (& self , args : & [& str]) -> Result < String > ; }
    };
}

RustcToolTrait!();