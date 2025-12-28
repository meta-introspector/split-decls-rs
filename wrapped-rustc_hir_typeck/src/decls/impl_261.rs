macro_rules! deps {
    () => {
        Candidate!();
        PickConstraintsForShadowed!();
        CandidateKind!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl PickConstraintsForShadowed { fn may_shadow_based_on_autoderefs (& self , autoderefs : usize) -> bool { autoderefs == self . autoderefs } fn candidate_may_shadow (& self , candidate : & Candidate < '_ >) -> bool { candidate . item . def_id != self . def_id && match candidate . kind { CandidateKind :: InherentImplCandidate { receiver_steps , .. } => match self . receiver_steps { Some (shadowed_receiver_steps) => receiver_steps > shadowed_receiver_steps , _ => false } , _ => false } } }
    };
}

impl_261!()