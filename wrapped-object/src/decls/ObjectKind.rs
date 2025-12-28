macro_rules! deps {
    () => {
        Dynamic!();
    };
}

macro_rules! ObjectKind {
    () => {
        deps!();
        # [doc = " An object kind."] # [doc = ""] # [doc = " Returned by [`Object::kind`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum ObjectKind { # [doc = " The object kind is unknown."] Unknown , # [doc = " Relocatable object."] Relocatable , # [doc = " Executable."] Executable , # [doc = " Dynamic shared object."] Dynamic , # [doc = " Core."] Core , }
    };
}

ObjectKind!()