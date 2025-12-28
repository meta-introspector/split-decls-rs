macro_rules! paxos_consensus {
    () => {
        # [proc_macro] # [decl2 (fn , name = "paxos_consensus" , vis = "pub" , hash = "71d8492f")] pub fn paxos_consensus (input : TokenStream) -> TokenStream { dao_governance :: paxos_consensus_impl (input) }
    };
}

paxos_consensus!()