macro_rules! deps {
    () => {
        AutoBorrow!();
        Adjustment!();
        AutoBorrowMutability!();
        Adjust!();
        AllowTwoPhase!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < 'db > Adjustment < 'db > { pub fn borrow (interner : DbInterner < 'db > , m : Mutability , ty : Ty < 'db > , lt : Region < 'db >) -> Self { let ty = Ty :: new_ref (interner , lt , ty , m) ; Adjustment { kind : Adjust :: Borrow (AutoBorrow :: Ref (AutoBorrowMutability :: new (m , AllowTwoPhase :: No))) , target : ty , } } }
    };
}

impl_59!()