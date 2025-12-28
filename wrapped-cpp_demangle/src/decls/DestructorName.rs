macro_rules! deps {
    () => {
        SimpleId!();
        Name!();
    };
}

macro_rules! DestructorName {
    () => {
        deps!();
        # [doc = " The `<destructor-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <destructor-name> ::= <unresolved-type> # e.g., ~T or ~decltype(f())"] # [doc = "                   ::= <simple-id>       # e.g., ~A<2*N>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum DestructorName { # [doc = " A destructor for an unresolved type."] Unresolved (UnresolvedTypeHandle) , # [doc = " A destructor for a resolved type name."] Name (SimpleId) , }
    };
}

DestructorName!()