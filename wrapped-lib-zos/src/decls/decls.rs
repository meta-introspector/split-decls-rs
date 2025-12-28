macro_rules! decls {
    () => {
        pub mod decls { include ! ("decls/_decl_module_invocation.rs") ; }
    };
}

decls!()