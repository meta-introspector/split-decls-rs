macro_rules! deps {
    () => {
        OrderingOrBool!();
        SizeHint!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        impl < T , F : FnMut (& T , & T) -> bool > OrderingOrBool < T , T > for F { type MergeResult = T ; fn left (left : T) -> Self :: MergeResult { left } fn right (right : T) -> Self :: MergeResult { right } fn merge (& mut self , left : T , right : T) -> (Option < Either < T , T > > , Self :: MergeResult) { if self (& left , & right) { (Some (Either :: Right (right)) , left) } else { (Some (Either :: Left (left)) , right) } } fn size_hint (left : SizeHint , right : SizeHint) -> SizeHint { size_hint :: add (left , right) } }
    };
}

impl_357!();