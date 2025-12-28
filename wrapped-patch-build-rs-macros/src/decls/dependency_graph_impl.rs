macro_rules! dependency_graph_impl {
    () => {
        # [decl2 (fn , name = "dependency_graph_impl" , vis = "pub" , hash = "c9ee9960")] pub fn dependency_graph_impl (_input : TokenStream) -> TokenStream { quote ! { { println ! ("cargo:warning=🕸️ Generating dependency graph") ; let graph = r#"
digraph RustcRing {
    rankdir=TB;
    node [shape=box, style=filled, fillcolor=lightblue];
    
    // Core compiler crates
    rustc_driver -> rustc_interface;
    rustc_interface -> rustc_middle;
    rustc_middle -> rustc_hir;
    rustc_hir -> rustc_ast;
    
    // Analysis crates  
    rustc_middle -> rustc_ty_utils;
    rustc_middle -> rustc_mir_build;
    rustc_middle -> rustc_const_eval;
    
    // Backend crates
    rustc_codegen_ssa -> rustc_target;
    rustc_codegen_llvm -> rustc_codegen_ssa;
    
    // The automorphic ring property
    rustc_ast -> rustc_expand [label="macros", color=red];
    rustc_expand -> rustc_ast [label="generates", color=red];
    
    label="Automorphic Ring of Rust\\nSelf-referential compiler structure";
}
            "# ; graph . to_string () } } . into () }
    };
}

dependency_graph_impl!();