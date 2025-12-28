macro_rules! deps {
    () => {
        MutBorrowKind!();
    };
}

macro_rules! BorrowKind {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Clone , Copy , PartialOrd , Ord)] pub enum BorrowKind { # [doc = " Data must be immutable and is aliasable."] Shared , # [doc = " The immediately borrowed place must be immutable, but projections from"] # [doc = " it don't need to be. For example, a shallow borrow of `a.b` doesn't"] # [doc = " conflict with a mutable borrow of `a.b.c`."] # [doc = ""] # [doc = " This is used when lowering matches: when matching on a place we want to"] # [doc = " ensure that place have the same value from the start of the match until"] # [doc = " an arm is selected. This prevents this code from compiling:"] # [doc = " ```compile_fail,E0510"] # [doc = " let mut x = &Some(0);"] # [doc = " match *x {"] # [doc = "     None => (),"] # [doc = "     Some(_) if { x = &None; false } => (),"] # [doc = "     Some(_) => (),"] # [doc = " }"] # [doc = " ```"] # [doc = " This can't be a shared borrow because mutably borrowing (*x as Some).0"] # [doc = " should not prevent `if let None = x { ... }`, for example, because the"] # [doc = " mutating `(*x as Some).0` can't affect the discriminant of `x`."] # [doc = " We can also report errors with this kind of borrow differently."] Shallow , # [doc = " Data is mutable and not aliasable."] Mut { kind : MutBorrowKind } , }
    };
}

BorrowKind!()