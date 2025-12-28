macro_rules! deps {
    () => {
        MergeLte!();
        SizeHint!();
        OrderingOrBool!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl < T : PartialOrd > OrderingOrBool < T , T > for MergeLte { type MergeResult = T ; fn left (left : T) -> Self :: MergeResult { left } fn right (right : T) -> Self :: MergeResult { right } fn merge (& mut self , left : T , right : T) -> (Option < Either < T , T > > , Self :: MergeResult) { if left <= right { (Some (Either :: Right (right)) , left) } else { (Some (Either :: Left (left)) , right) } } fn size_hint (left : SizeHint , right : SizeHint) -> SizeHint { size_hint :: add (left , right) } }
    };
}

impl_358!();