macro_rules! deps {
    () => {
        BodyWithBorrowckFacts!();
        ConsumerOptions!();
    };
}

macro_rules! BorrowckConsumer {
    () => {
        deps!();
        # [doc = " Struct used during mir borrowck to collect bodies with facts for a typeck root and all"] # [doc = " its nested bodies."] pub (crate) struct BorrowckConsumer < 'tcx > { options : ConsumerOptions , bodies : FxHashMap < LocalDefId , BodyWithBorrowckFacts < 'tcx > > , }
    };
}

BorrowckConsumer!()