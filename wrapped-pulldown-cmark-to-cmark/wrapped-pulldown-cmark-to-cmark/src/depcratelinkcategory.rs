// Generated macro for LinkCategory (enum)
macro_rules! DepcrateLinkCategory {
() => {
// Module: crate
// Provides: {"LinkCategory"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum LinkCategory < 'a > { AngleBracketed , Reference { uri : Cow < 'a , str > , title : Cow < 'a , str > , id : Cow < 'a , str > , } , Collapsed { uri : Cow < 'a , str > , title : Cow < 'a , str > , } , Shortcut { uri : Cow < 'a , str > , title : Cow < 'a , str > , } , Other { uri : Cow < 'a , str > , title : Cow < 'a , str > , } , }
};
}
