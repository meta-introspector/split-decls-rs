macro_rules! VisitMode {
    () => {
        # [derive (Copy , Clone , Eq , PartialEq)] pub (crate) enum VisitMode { Normal , Inline , }
    };
}

VisitMode!()