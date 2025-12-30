// Generated macro for python_command (function)
macro_rules! Depcrate_external_deps_pythonpython_command {
() => {
// Module: crate::external_deps::python
// Provides: {"python_command"}
// Dependencies: {}
# [doc = " Obtain path of python as provided by the `PYTHON` environment variable. It is up to the caller"] # [doc = " to document and check if the python version is compatible with its intended usage."] # [track_caller] # [must_use] pub fn python_command () -> Command { let python_path = env_var ("PYTHON") ; Command :: new (python_path) }
};
}
