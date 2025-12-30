// Generated macro for location (module)
macro_rules! Depcrate_repository_thread_safelocation {
() => {
// Module: crate::repository::thread_safe
// Provides: {"location"}
// Dependencies: {}
mod location { impl crate :: ThreadSafeRepository { # [doc = " The path to the `.git` directory itself, or equivalent if this is a bare repository."] pub fn path (& self) -> & std :: path :: Path { self . git_dir () } # [doc = " Return the path to the repository itself, containing objects, references, configuration, and more."] # [doc = ""] # [doc = " Synonymous to [`path()`][crate::ThreadSafeRepository::path()]."] pub fn git_dir (& self) -> & std :: path :: Path { self . refs . git_dir () } # [doc = " Return the path to the working directory if this is not a bare repository."] pub fn work_dir (& self) -> Option < & std :: path :: Path > { self . work_tree . as_deref () } # [doc = " Return the path to the directory containing all objects."] pub fn objects_dir (& self) -> & std :: path :: Path { self . objects . path () } } }
};
}
