macro_rules! deps {
    () => {
        MkFile!();
    };
}

macro_rules! Container {
    () => {
        deps!();
        # [doc = " A builder for configuring a container to run."] pub struct Container { # [doc = " The host directory that forms the basis of the Docker image."] build_context : PathBuf , # [doc = " Files to copy over to the image."] files : Vec < MkFile > , }
    };
}

Container!()