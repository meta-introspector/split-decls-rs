macro_rules! zk_witness {
    () => {
        # [proc_macro] # [decl2 (fn , name = "zk_witness" , vis = "pub" , hash = "4cb7cb6f")] pub fn zk_witness (input : TokenStream) -> TokenStream { zk_proof :: zk_witness_impl (input) }
    };
}

zk_witness!()