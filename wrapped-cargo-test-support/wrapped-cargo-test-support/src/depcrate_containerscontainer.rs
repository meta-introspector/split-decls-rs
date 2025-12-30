// Generated macro for Container (struct)
macro_rules! Depcrate_containersContainer {
() => {
// Module: crate::containers
// Provides: {"Container"}
// Dependencies: {}
# [doc = " A builder for configuring a container to run."] pub struct Container { # [doc = " The host directory that forms the basis of the Docker image."] build_context : PathBuf , # [doc = " Files to copy over to the image."] files : Vec < MkFile > , }
};
}
