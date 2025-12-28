macro_rules! deps {
    () => {
        InitializationRequiringAction!();
    };
}

macro_rules! impl_547 {
    () => {
        deps!();
        impl InitializationRequiringAction { fn as_noun (self) -> & 'static str { match self { InitializationRequiringAction :: Borrow => "borrow" , InitializationRequiringAction :: MatchOn => "use" , InitializationRequiringAction :: Use => "use" , InitializationRequiringAction :: Assignment => "assign" , InitializationRequiringAction :: PartialAssignment => "assign to part" , } } fn as_verb_in_past_tense (self) -> & 'static str { match self { InitializationRequiringAction :: Borrow => "borrowed" , InitializationRequiringAction :: MatchOn => "matched on" , InitializationRequiringAction :: Use => "used" , InitializationRequiringAction :: Assignment => "assigned" , InitializationRequiringAction :: PartialAssignment => "partially assigned" , } } fn as_general_verb_in_past_tense (self) -> & 'static str { match self { InitializationRequiringAction :: Borrow | InitializationRequiringAction :: MatchOn | InitializationRequiringAction :: Use => "used" , InitializationRequiringAction :: Assignment => "assigned" , InitializationRequiringAction :: PartialAssignment => "partially assigned" , } } }
    };
}

impl_547!()