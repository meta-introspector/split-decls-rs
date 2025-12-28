macro_rules! deps {
    () => {
        SizeHint!();
    };
}

macro_rules! OrderingOrBool {
    () => {
        deps!();
        pub trait OrderingOrBool < L , R > { type MergeResult ; fn left (left : L) -> Self :: MergeResult ; fn right (right : R) -> Self :: MergeResult ; fn merge (& mut self , left : L , right : R) -> (Option < Either < L , R > > , Self :: MergeResult) ; fn size_hint (left : SizeHint , right : SizeHint) -> SizeHint ; }
    };
}

OrderingOrBool!();