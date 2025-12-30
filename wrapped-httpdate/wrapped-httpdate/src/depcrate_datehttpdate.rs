// Generated macro for HttpDate (struct)
macro_rules! Depcrate_dateHttpDate {
() => {
// Module: crate::date
// Provides: {"HttpDate"}
// Dependencies: {}
# [doc = " HTTP timestamp type."] # [doc = ""] # [doc = " Parse using `FromStr` impl."] # [doc = " Format using the `Display` trait."] # [doc = " Convert timestamp into/from `SytemTime` to use."] # [doc = " Supports comparsion and sorting."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub struct HttpDate { # [doc = " 0...59"] sec : u8 , # [doc = " 0...59"] min : u8 , # [doc = " 0...23"] hour : u8 , # [doc = " 1...31"] day : u8 , # [doc = " 1...12"] mon : u8 , # [doc = " 1970...9999"] year : u16 , # [doc = " 1...7"] wday : u8 , }
};
}
