macro_rules! deps {
    () => {
        BorrowckConsumer!();
        BodyWithBorrowckFacts!();
        ConsumerOptions!();
    };
}

macro_rules! impl_510 {
    () => {
        deps!();
        impl < 'tcx > BorrowckConsumer < 'tcx > { pub (crate) fn new (options : ConsumerOptions) -> Self { Self { options , bodies : Default :: default () } } pub (crate) fn insert_body (& mut self , def_id : LocalDefId , body : BodyWithBorrowckFacts < 'tcx >) { if self . bodies . insert (def_id , body) . is_some () { bug ! ("unexpected previous body for {def_id:?}") ; } } # [doc = " Should the Polonius input facts be computed?"] pub (crate) fn polonius_input (& self) -> bool { matches ! (self . options , ConsumerOptions :: PoloniusInputFacts | ConsumerOptions :: PoloniusOutputFacts) } # [doc = " Should we run Polonius and collect the output facts?"] pub (crate) fn polonius_output (& self) -> bool { matches ! (self . options , ConsumerOptions :: PoloniusOutputFacts) } }
    };
}

impl_510!()