macro_rules! deps {
    () => {
        LoanInvalidationsGenerator!();
        BorrowSet!();
        PoloniusLocationTable!();
        PoloniusFacts!();
    };
}

macro_rules! emit_loan_invalidations {
    () => {
        deps!();
        # [doc = " Emit `loan_invalidated_at` facts."] pub (super) fn emit_loan_invalidations < 'tcx > (tcx : TyCtxt < 'tcx > , facts : & mut PoloniusFacts , body : & Body < 'tcx > , location_table : & PoloniusLocationTable , borrow_set : & BorrowSet < 'tcx > ,) { let dominators = body . basic_blocks . dominators () ; let mut visitor = LoanInvalidationsGenerator { facts , borrow_set , tcx , location_table , body , dominators } ; visitor . visit_body (body) ; }
    };
}

emit_loan_invalidations!()