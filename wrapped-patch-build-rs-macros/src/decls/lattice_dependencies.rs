macro_rules! lattice_dependencies {
    () => {
        # [proc_macro] # [decl2 (fn , name = "lattice_dependencies" , vis = "pub" , hash = "6283aa32")] pub fn lattice_dependencies (input : TokenStream) -> TokenStream { macro_lattice :: lattice_dependencies_impl (input) }
    };
}

lattice_dependencies!();