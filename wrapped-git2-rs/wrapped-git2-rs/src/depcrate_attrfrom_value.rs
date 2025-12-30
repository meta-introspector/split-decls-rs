// Generated macro for from_value (macro)
macro_rules! Depcrate_attrfrom_value {
() => {
// Module: crate::attr
// Provides: {"from_value"}
// Dependencies: {}
macro_rules ! from_value { ($ value : expr => $ string : expr) => { match unsafe { raw :: git_attr_value ($ value . map_or (ptr :: null () , | v | v . as_ptr () . cast ())) } { raw :: GIT_ATTR_VALUE_TRUE => Self :: True , raw :: GIT_ATTR_VALUE_FALSE => Self :: False , raw :: GIT_ATTR_VALUE_STRING => $ string , raw :: GIT_ATTR_VALUE_UNSPECIFIED => Self :: Unspecified , _ => unreachable ! () , } } ; }
};
}
