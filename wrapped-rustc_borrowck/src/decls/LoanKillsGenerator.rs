macro_rules! deps {
    () => {
        PoloniusLocationTable!();
        BorrowSet!();
        PoloniusFacts!();
    };
}

macro_rules! LoanKillsGenerator {
    () => {
        deps!();
        struct LoanKillsGenerator < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , facts : & 'a mut PoloniusFacts , location_table : & 'a PoloniusLocationTable , borrow_set : & 'a BorrowSet < 'tcx > , body : & 'a Body < 'tcx > , }
    };
}

LoanKillsGenerator!();