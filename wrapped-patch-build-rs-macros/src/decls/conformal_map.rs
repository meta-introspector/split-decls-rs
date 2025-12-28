macro_rules! conformal_map {
    () => {
        # [proc_macro] # [decl2 (fn , name = "conformal_map" , vis = "pub" , hash = "5a695325")] pub fn conformal_map (input : TokenStream) -> TokenStream { lmfdb_morph :: conformal_map_impl (input) }
    };
}

conformal_map!()