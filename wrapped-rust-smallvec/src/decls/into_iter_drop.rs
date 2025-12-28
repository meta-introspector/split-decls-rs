macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! into_iter_drop {
    () => {
        deps!();
        # [test] fn into_iter_drop () { use std :: cell :: Cell ; struct DropCounter < 'a > (& 'a Cell < i32 >) ; impl < 'a > Drop for DropCounter < 'a > { fn drop (& mut self) { self . 0 . set (self . 0 . get () + 1) ; } } { let cell = Cell :: new (0) ; let mut v : SmallVec < DropCounter < '_ > , 2 > = SmallVec :: new () ; v . push (DropCounter (& cell)) ; v . into_iter () ; assert_eq ! (cell . get () , 1) ; } { let cell = Cell :: new (0) ; let mut v : SmallVec < DropCounter < '_ > , 2 > = SmallVec :: new () ; v . push (DropCounter (& cell)) ; v . push (DropCounter (& cell)) ; assert ! (v . into_iter () . next () . is_some ()) ; assert_eq ! (cell . get () , 2) ; } { let cell = Cell :: new (0) ; let mut v : SmallVec < DropCounter < '_ > , 2 > = SmallVec :: new () ; v . push (DropCounter (& cell)) ; v . push (DropCounter (& cell)) ; v . push (DropCounter (& cell)) ; assert ! (v . into_iter () . next () . is_some ()) ; assert_eq ! (cell . get () , 3) ; } { let cell = Cell :: new (0) ; let mut v : SmallVec < DropCounter < '_ > , 2 > = SmallVec :: new () ; v . push (DropCounter (& cell)) ; v . push (DropCounter (& cell)) ; v . push (DropCounter (& cell)) ; { let mut it = v . into_iter () ; assert ! (it . next () . is_some ()) ; assert ! (it . next_back () . is_some ()) ; } assert_eq ! (cell . get () , 3) ; } }
    };
}

into_iter_drop!();