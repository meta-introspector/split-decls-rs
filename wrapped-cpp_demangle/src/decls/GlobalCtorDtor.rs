macro_rules! deps {
    () => {
        MangledName!();
    };
}

macro_rules! GlobalCtorDtor {
    () => {
        deps!();
        # [doc = " A global constructor or destructor."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum GlobalCtorDtor { # [doc = " A global constructor."] Ctor (Box < MangledName >) , # [doc = " A global destructor."] Dtor (Box < MangledName >) , }
    };
}

GlobalCtorDtor!();