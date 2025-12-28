macro_rules! AncestorsIter {
    () => {
        pub type AncestorsIter < Find > = gix_traverse :: commit :: Simple < Find , fn (& gix_hash :: oid) -> bool > ;
    };
}

AncestorsIter!()