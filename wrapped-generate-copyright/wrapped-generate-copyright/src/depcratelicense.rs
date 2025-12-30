// Generated macro for License (struct)
macro_rules! DepcrateLicense {
() => {
// Module: crate
// Provides: {"License"}
// Dependencies: {}
# [doc = " A License has an SPDX license name and a list of copyright holders."] # [derive (serde :: Deserialize , Clone , Debug , PartialEq , Eq)] struct License { spdx : String , copyright : Vec < String > , }
};
}
