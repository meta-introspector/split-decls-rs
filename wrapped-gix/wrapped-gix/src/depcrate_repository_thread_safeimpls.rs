// Generated macro for impls (module)
macro_rules! Depcrate_repository_thread_safeimpls {
() => {
// Module: crate::repository::thread_safe
// Provides: {"impls"}
// Dependencies: {}
mod impls { impl std :: fmt :: Debug for crate :: ThreadSafeRepository { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Repository(git = '{}', working_tree: {:?}" , self . git_dir () . display () , self . work_tree) } } impl PartialEq < crate :: ThreadSafeRepository > for crate :: ThreadSafeRepository { fn eq (& self , other : & crate :: ThreadSafeRepository) -> bool { self . git_dir () == other . git_dir () && self . work_tree == other . work_tree } } }
};
}
