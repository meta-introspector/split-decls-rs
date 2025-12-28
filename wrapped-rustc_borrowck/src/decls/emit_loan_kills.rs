macro_rules! deps {
    () => {
        PoloniusFacts!();
        PoloniusLocationTable!();
        LoanKillsGenerator!();
        BorrowSet!();
    };
}

macro_rules! emit_loan_kills {
    () => {
        deps!();
        # [doc = " Emit `loan_killed_at` and `cfg_edge` facts at the same time."] pub (super) fn emit_loan_kills < 'tcx > (tcx : TyCtxt < 'tcx > , facts : & mut PoloniusFacts , body : & Body < 'tcx > , location_table : & PoloniusLocationTable , borrow_set : & BorrowSet < 'tcx > ,) { let mut visitor = LoanKillsGenerator { borrow_set , tcx , location_table , facts , body } ; for (bb , data) in body . basic_blocks . iter_enumerated () { visitor . visit_basic_block_data (bb , data) ; } }
    };
}

emit_loan_kills!();