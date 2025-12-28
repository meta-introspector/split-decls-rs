macro_rules! macro_lattice {
    () => {
        # [proc_macro] # [decl2 (fn , name = "macro_lattice" , vis = "pub" , hash = "b2195169")] pub fn macro_lattice (input : TokenStream) -> TokenStream { macro_lattice :: macro_lattice_impl (input) }
    };
}

macro_lattice!();