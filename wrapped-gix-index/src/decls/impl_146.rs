macro_rules! deps {
    () => {
        Signature!();
        Extensions!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl Extensions { # [doc = " Returns `Some(signature)` if it should be written out."] pub fn should_write (& self , signature : extension :: Signature) -> Option < extension :: Signature > { match self { Extensions :: None => None , Extensions :: All => Some (signature) , Extensions :: Given { tree_cache , end_of_index_entry , } => match signature { extension :: tree :: SIGNATURE => tree_cache , extension :: end_of_index_entry :: SIGNATURE => end_of_index_entry , _ => & false , } . then (| | signature) , } } }
    };
}

impl_146!();