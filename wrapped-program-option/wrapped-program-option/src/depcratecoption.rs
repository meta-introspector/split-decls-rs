// Generated macro for COption (enum)
macro_rules! DepcrateCOption {
() => {
// Module: crate
// Provides: {"COption"}
// Dependencies: {}
# [doc = " A C representation of Rust's `std::option::Option`"] # [repr (C)] # [derive (Copy , PartialEq , PartialOrd , Eq , Ord , Debug , Hash)] pub enum COption < T > { # [doc = " No value"] None , # [doc = " Some value `T`"] Some (T) , }
};
}
