// Generated macro for ImageLink (enum)
macro_rules! DepcrateImageLink {
() => {
// Module: crate
// Provides: {"ImageLink"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum ImageLink < 'a > { Reference { uri : Cow < 'a , str > , title : Cow < 'a , str > , id : Cow < 'a , str > , } , Collapsed { uri : Cow < 'a , str > , title : Cow < 'a , str > , } , Shortcut { uri : Cow < 'a , str > , title : Cow < 'a , str > , } , Other { uri : Cow < 'a , str > , title : Cow < 'a , str > , } , }
};
}
