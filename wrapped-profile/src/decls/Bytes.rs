macro_rules! Bytes {
    () => {
        # [derive (Default , PartialEq , Eq , PartialOrd , Ord , Hash , Clone , Copy)] pub struct Bytes (isize) ;
    };
}

Bytes!();