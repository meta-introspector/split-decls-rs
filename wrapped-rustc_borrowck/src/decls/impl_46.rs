macro_rules! deps {
    () => {
        BorrowckDomain!();
        Borrowck!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'a , 'tcx > Analysis < 'tcx > for Borrowck < 'a , 'tcx > { type Domain = BorrowckDomain ; const NAME : & 'static str = "borrowck" ; fn bottom_value (& self , body : & mir :: Body < 'tcx >) -> Self :: Domain { BorrowckDomain { borrows : self . borrows . bottom_value (body) , uninits : self . uninits . bottom_value (body) , ever_inits : self . ever_inits . bottom_value (body) , } } fn initialize_start_block (& self , _body : & mir :: Body < 'tcx > , _state : & mut Self :: Domain) { unreachable ! () ; } fn apply_early_statement_effect (& mut self , state : & mut Self :: Domain , stmt : & mir :: Statement < 'tcx > , loc : Location ,) { self . borrows . apply_early_statement_effect (& mut state . borrows , stmt , loc) ; self . uninits . apply_early_statement_effect (& mut state . uninits , stmt , loc) ; self . ever_inits . apply_early_statement_effect (& mut state . ever_inits , stmt , loc) ; } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , stmt : & mir :: Statement < 'tcx > , loc : Location ,) { self . borrows . apply_primary_statement_effect (& mut state . borrows , stmt , loc) ; self . uninits . apply_primary_statement_effect (& mut state . uninits , stmt , loc) ; self . ever_inits . apply_primary_statement_effect (& mut state . ever_inits , stmt , loc) ; } fn apply_early_terminator_effect (& mut self , state : & mut Self :: Domain , term : & mir :: Terminator < 'tcx > , loc : Location ,) { self . borrows . apply_early_terminator_effect (& mut state . borrows , term , loc) ; self . uninits . apply_early_terminator_effect (& mut state . uninits , term , loc) ; self . ever_inits . apply_early_terminator_effect (& mut state . ever_inits , term , loc) ; } fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , term : & 'mir mir :: Terminator < 'tcx > , loc : Location ,) -> TerminatorEdges < 'mir , 'tcx > { self . borrows . apply_primary_terminator_effect (& mut state . borrows , term , loc) ; self . uninits . apply_primary_terminator_effect (& mut state . uninits , term , loc) ; self . ever_inits . apply_primary_terminator_effect (& mut state . ever_inits , term , loc) ; TerminatorEdges :: None } fn apply_call_return_effect (& mut self , _state : & mut Self :: Domain , _block : BasicBlock , _return_places : CallReturnPlaces < '_ , 'tcx > ,) { unreachable ! () ; } }
    };
}

impl_46!()