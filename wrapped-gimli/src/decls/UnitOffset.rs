macro_rules! UnitOffset {
    () => {
        # [doc = " An offset into the current compilation or type unit."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Ord , PartialOrd , Hash)] pub struct UnitOffset < T = usize > (pub T) ;
    };
}

UnitOffset!()