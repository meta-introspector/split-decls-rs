macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! Row {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq , Ord , PartialOrd)] pub struct Row { file : * const File , index : usize , }
    };
}

Row!();