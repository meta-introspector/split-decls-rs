macro_rules! Rustc {
    () => {
        # [derive (Clone , Debug)] pub struct Rustc { rustc : PathBuf , rustc_wrapper : Option < PathBuf > , rustc_workspace_wrapper : Option < PathBuf > , }
    };
}

Rustc!();