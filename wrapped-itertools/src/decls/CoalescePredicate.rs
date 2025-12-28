macro_rules! CoalescePredicate {
    () => {
        pub trait CoalescePredicate < Item , T > { fn coalesce_pair (& mut self , t : T , item : Item) -> Result < T , (T , T) > ; }
    };
}

CoalescePredicate!()