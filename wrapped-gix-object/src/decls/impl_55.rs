macro_rules! deps {
    () => {
        Tree!();
        TreeRef!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl From < TreeRef < '_ > > for Tree { fn from (other : TreeRef < '_ >) -> Tree { let TreeRef { entries } = other ; Tree { entries : entries . into_iter () . map (Into :: into) . collect () , } } }
    };
}

impl_55!()