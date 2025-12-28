macro_rules! deps {
    () => {
        TransferFunction!();
        Operand!();
        Place!();
        Qualif!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < 'tcx , Q > Visitor < 'tcx > for TransferFunction < '_ , 'tcx , Q > where Q : Qualif , { fn visit_operand (& mut self , operand : & mir :: Operand < 'tcx > , location : Location) { self . super_operand (operand , location) ; if ! Q :: IS_CLEARED_ON_MOVE { return ; } if let mir :: Operand :: Move (place) = operand && let Some (local) = place . as_local () { if ! self . state . borrow . contains (local) { self . state . qualif . remove (local) ; } } } fn visit_assign (& mut self , place : & mir :: Place < 'tcx > , rvalue : & mir :: Rvalue < 'tcx > , location : Location ,) { let qualif = qualifs :: in_rvalue :: < Q , _ > (self . ccx , & mut | l | self . state . qualif . contains (l) , rvalue) ; if ! place . is_indirect () { self . assign_qualif_direct (place , qualif) ; } self . super_assign (place , rvalue , location) ; } fn visit_rvalue (& mut self , rvalue : & mir :: Rvalue < 'tcx > , location : Location) { self . super_rvalue (rvalue , location) ; match rvalue { mir :: Rvalue :: RawPtr (_mt , borrowed_place) => { if ! borrowed_place . is_indirect () && self . address_of_allows_mutation () { let place_ty = borrowed_place . ty (self . ccx . body , self . ccx . tcx) . ty ; if Q :: in_any_value_of_ty (self . ccx , place_ty) { self . state . qualif . insert (borrowed_place . local) ; self . state . borrow . insert (borrowed_place . local) ; } } } mir :: Rvalue :: Ref (_ , kind , borrowed_place) => { if ! borrowed_place . is_indirect () && self . ref_allows_mutation (* kind , * borrowed_place) { let place_ty = borrowed_place . ty (self . ccx . body , self . ccx . tcx) . ty ; if Q :: in_any_value_of_ty (self . ccx , place_ty) { self . state . qualif . insert (borrowed_place . local) ; self . state . borrow . insert (borrowed_place . local) ; } } } mir :: Rvalue :: Cast (..) | mir :: Rvalue :: ShallowInitBox (..) | mir :: Rvalue :: Use (..) | mir :: Rvalue :: CopyForDeref (..) | mir :: Rvalue :: ThreadLocalRef (..) | mir :: Rvalue :: Repeat (..) | mir :: Rvalue :: Len (..) | mir :: Rvalue :: BinaryOp (..) | mir :: Rvalue :: NullaryOp (..) | mir :: Rvalue :: UnaryOp (..) | mir :: Rvalue :: Discriminant (..) | mir :: Rvalue :: Aggregate (..) | mir :: Rvalue :: WrapUnsafeBinder (..) => { } } } fn visit_statement (& mut self , statement : & Statement < 'tcx > , location : Location) { match statement . kind { StatementKind :: StorageDead (local) => { self . state . qualif . remove (local) ; self . state . borrow . remove (local) ; } _ => self . super_statement (statement , location) , } } fn visit_terminator (& mut self , terminator : & mir :: Terminator < 'tcx > , location : Location) { self . super_terminator (terminator , location) ; } }
    };
}

impl_67!()