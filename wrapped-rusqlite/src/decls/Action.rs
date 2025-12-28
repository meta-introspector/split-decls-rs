macro_rules! Action {
    () => {
        # [doc = " Action Codes"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] # [repr (i32)] # [non_exhaustive] pub enum Action { # [doc = " Unsupported / unexpected action"] UNKNOWN = - 1 , # [doc = " DELETE command"] SQLITE_DELETE = ffi :: SQLITE_DELETE , # [doc = " INSERT command"] SQLITE_INSERT = ffi :: SQLITE_INSERT , # [doc = " UPDATE command"] SQLITE_UPDATE = ffi :: SQLITE_UPDATE , }
    };
}

Action!();