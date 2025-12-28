macro_rules! deps {
    () => {
        Locals!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        impl < 'tcx , V > Locals < 'tcx , V > { pub (super) fn empty () -> Locals < 'tcx , V > { Locals { values : IndexVec :: default () } } pub (super) fn indices (& self) -> impl DoubleEndedIterator < Item = mir :: Local > + Clone + 'tcx { self . values . indices () } }
    };
}

impl_487!();