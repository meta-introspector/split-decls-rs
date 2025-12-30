// Generated macro for STD_PUBLIC_CRATES (const)
macro_rules! Depcrate_core_build_steps_docSTD_PUBLIC_CRATES {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"STD_PUBLIC_CRATES"}
// Dependencies: {}
# [doc = " Name of the crates that are visible to consumers of the standard library."] # [doc = " Documentation for internal crates is handled by the rustc step, so internal crates will show"] # [doc = " up there."] # [doc = ""] # [doc = " Order here is important!"] # [doc = " Crates need to be processed starting from the leaves, otherwise rustdoc will not"] # [doc = " create correct links between crates because rustdoc depends on the"] # [doc = " existence of the output directories to know if it should be a local"] # [doc = " or remote link."] const STD_PUBLIC_CRATES : [& str ; 5] = ["core" , "alloc" , "std" , "proc_macro" , "test"] ;
};
}
