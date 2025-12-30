// Generated macro for macro_9213 (macro)
macro_rules! Depcrate_rc_clone_in_vec_initmacro_9213 {
() => {
// Module: crate::rc_clone_in_vec_init
// Provides: {"macro_9213"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for reference-counted pointers (`Arc`, `Rc`, `rc::Weak`, and `sync::Weak`)"] # [doc = " in `vec![elem; len]`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This will create `elem` once and clone it `len` times - doing so with `Arc`/`Rc`/`Weak`"] # [doc = " is a bit misleading, as it will create references to the same pointer, rather"] # [doc = " than different instances."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let v = vec![std::sync::Arc::new(\"some data\".to_string()); 100];"] # [doc = " // or"] # [doc = " let v = vec![std::rc::Rc::new(\"some data\".to_string()); 100];"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " // Initialize each value separately:"] # [doc = " let mut data = Vec::with_capacity(100);"] # [doc = " for _ in 0..100 {"] # [doc = "     data.push(std::rc::Rc::new(\"some data\".to_string()));"] # [doc = " }"] # [doc = ""] # [doc = " // Or if you want clones of the same reference,"] # [doc = " // Create the reference beforehand to clarify that"] # [doc = " // it should be cloned for each value"] # [doc = " let data = std::rc::Rc::new(\"some data\".to_string());"] # [doc = " let v = vec![data; 100];"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub RC_CLONE_IN_VEC_INIT , suspicious , "initializing reference-counted pointer in `vec![elem; len]`" }
};
}
