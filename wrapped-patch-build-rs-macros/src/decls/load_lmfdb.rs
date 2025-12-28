macro_rules! load_lmfdb {
    () => {
        # [proc_macro] # [decl2 (fn , name = "load_lmfdb" , vis = "pub" , hash = "579a101f")] pub fn load_lmfdb (input : TokenStream) -> TokenStream { lmfdb_morph :: load_lmfdb_impl (input) }
    };
}

load_lmfdb!()