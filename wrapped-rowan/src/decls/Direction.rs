macro_rules! Direction {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum Direction { Next , Prev , }
    };
}

Direction!()