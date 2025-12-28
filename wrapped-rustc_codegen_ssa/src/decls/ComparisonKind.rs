macro_rules! ComparisonKind {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq)] pub enum ComparisonKind { Exact , AtLeast , }
    };
}

ComparisonKind!()