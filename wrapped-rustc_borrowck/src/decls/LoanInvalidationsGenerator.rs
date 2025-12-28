macro_rules! deps {
    () => {
        PoloniusLocationTable!();
        PoloniusFacts!();
        BorrowSet!();
    };
}

macro_rules! LoanInvalidationsGenerator {
    () => {
        deps!();
        struct LoanInvalidationsGenerator < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , facts : & 'a mut PoloniusFacts , body : & 'a Body < 'tcx > , location_table : & 'a PoloniusLocationTable , dominators : & 'a Dominators < BasicBlock > , borrow_set : & 'a BorrowSet < 'tcx > , }
    };
}

LoanInvalidationsGenerator!();