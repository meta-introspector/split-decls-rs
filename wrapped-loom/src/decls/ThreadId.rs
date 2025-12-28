macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! ThreadId {
    () => {
        deps!();
        # [doc = " Mock implementation of `std::thread::ThreadId`."] # [derive (Clone , Copy , Eq , Hash , PartialEq)] pub struct ThreadId { id : crate :: rt :: thread :: Id , }
    };
}

ThreadId!();