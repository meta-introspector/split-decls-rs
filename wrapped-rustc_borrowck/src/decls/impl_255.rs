macro_rules! deps {
    () => {
        RustcFacts!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl polonius_engine :: FactTypes for RustcFacts { type Origin = PoloniusRegionVid ; type Loan = BorrowIndex ; type Point = LocationIndex ; type Variable = Local ; type Path = MovePathIndex ; }
    };
}

impl_255!()