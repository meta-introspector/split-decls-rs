macro_rules! deps {
    () => {
        AssocItem!();
    };
}

macro_rules! PathCandidateCallback {
    () => {
        deps!();
        pub trait PathCandidateCallback { fn on_inherent_item (& mut self , item : AssocItem) -> ControlFlow < () > ; fn on_trait_item (& mut self , item : AssocItem) -> ControlFlow < () > ; }
    };
}

PathCandidateCallback!();