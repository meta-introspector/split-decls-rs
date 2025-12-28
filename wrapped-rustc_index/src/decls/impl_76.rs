macro_rules! deps {
    () => {
        SparseIntervalMatrix!();
        IndexVec!();
        Idx!();
        IntervalSet!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < R : Idx , C : Step + Idx > SparseIntervalMatrix < R , C > { pub fn new (column_size : usize) -> SparseIntervalMatrix < R , C > { SparseIntervalMatrix { rows : IndexVec :: new () , column_size } } pub fn rows (& self) -> impl Iterator < Item = R > { self . rows . indices () } pub fn row (& self , row : R) -> Option < & IntervalSet < C > > { self . rows . get (row) } fn ensure_row (& mut self , row : R) -> & mut IntervalSet < C > { self . rows . ensure_contains_elem (row , | | IntervalSet :: new (self . column_size)) } pub fn union_row (& mut self , row : R , from : & IntervalSet < C >) -> bool where C : Step , { self . ensure_row (row) . union (from) } pub fn union_rows (& mut self , read : R , write : R) -> bool where C : Step , { if read == write || self . rows . get (read) . is_none () { return false ; } self . ensure_row (write) ; let (read_row , write_row) = self . rows . pick2_mut (read , write) ; write_row . union (read_row) } pub fn insert_all_into_row (& mut self , row : R) { self . ensure_row (row) . insert_all () ; } pub fn insert_range (& mut self , row : R , range : impl RangeBounds < C > + Clone) { self . ensure_row (row) . insert_range (range) ; } pub fn insert (& mut self , row : R , point : C) -> bool { self . ensure_row (row) . insert (point) } pub fn append (& mut self , row : R , point : C) { self . ensure_row (row) . append (point) } pub fn contains (& self , row : R , point : C) -> bool { self . row (row) . is_some_and (| r | r . contains (point)) } }
    };
}

impl_76!();