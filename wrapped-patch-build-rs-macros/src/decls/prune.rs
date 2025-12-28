macro_rules! prune {
    () => {
        # [proc_macro] # [decl2 (fn , name = "prune" , vis = "pub" , hash = "de877f0b")] pub fn prune (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let data = input_str . value () ; quote ! { { let pruned = # data . lines () . filter (| line | ! line . contains ("target/")) . filter (| line | ! line . contains (".git/")) . filter (| line | ! line . contains ("node_modules/")) . take (1000) . collect ::< Vec < _ >> () . join ("\n") ; println ! ("cargo:warning=✂️ Pruned: {} -> {} lines" , # data . lines () . count () , pruned . lines () . count ()) ; pruned } } . into () }
    };
}

prune!()