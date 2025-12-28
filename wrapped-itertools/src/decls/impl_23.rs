macro_rules! deps {
    () => {
        CoalescePredicate!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < F , Item , T > CoalescePredicate < Item , T > for F where F : FnMut (T , Item) -> Result < T , (T , T) > , { fn coalesce_pair (& mut self , t : T , item : Item) -> Result < T , (T , T) > { self (t , item) } }
    };
}

impl_23!();