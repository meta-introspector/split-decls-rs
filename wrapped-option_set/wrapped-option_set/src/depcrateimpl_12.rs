// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl CaseTransform { # [doc = " Converts the name of an option flag to the specified case"] fn apply (self , s : & str) -> Cow < str > { use CaseTransform :: * ; match self { Identity => Cow :: Borrowed (s) , LowerSnake => Cow :: Owned (s . to_snake_case ()) , UpperSnake => Cow :: Owned (s . to_shouty_snake_case ()) , LowerCamel => Cow :: Owned (s . to_lower_camel_case ()) , UpperCamel => Cow :: Owned (s . to_upper_camel_case ()) , Kebab => Cow :: Owned (s . to_kebab_case ()) , Title => Cow :: Owned (s . to_title_case ()) , } } }
};
}
