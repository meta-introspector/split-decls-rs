macro_rules! deps {
    () => {
        SelectionField!();
        Lookahead!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'a > From < SelectionField < 'a > > for Lookahead < 'a > { fn from (selection_field : SelectionField < 'a >) -> Self { Lookahead { fragments : selection_field . fragments , fields : vec ! [selection_field . field] , context : selection_field . context , } } }
    };
}

impl_71!();