macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! LinkingFailed {
    () => {
        deps!();
        pub (crate) struct LinkingFailed < 'a > { pub linker_path : & 'a Path , pub exit_status : ExitStatus , pub command : Command , pub escaped_output : String , pub verbose : bool , pub sysroot_dir : PathBuf , }
    };
}

LinkingFailed!();