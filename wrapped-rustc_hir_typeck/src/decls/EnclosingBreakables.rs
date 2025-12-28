macro_rules! deps {
    () => {
        BreakableCtxt!();
    };
}

macro_rules! EnclosingBreakables {
    () => {
        deps!();
        pub struct EnclosingBreakables < 'tcx > { stack : Vec < BreakableCtxt < 'tcx > > , by_id : HirIdMap < usize > , }
    };
}

EnclosingBreakables!()