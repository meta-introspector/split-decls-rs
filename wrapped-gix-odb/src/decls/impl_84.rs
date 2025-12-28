macro_rules! deps {
    () => {
        Either!();
        Ordering!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        # [allow (clippy :: non_canonical_partial_ord_impl)] impl PartialOrd < Self > for Either { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . path () . cmp (other . path ())) } }
    };
}

impl_84!()