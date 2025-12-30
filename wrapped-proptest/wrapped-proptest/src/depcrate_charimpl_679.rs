// Generated macro for impl_679 (impl)
macro_rules! Depcrate_charimpl_679 {
() => {
// Module: crate::char
// Provides: {"impl_679"}
// Dependencies: {}
impl < 'a > CharStrategy < 'a > { # [doc = " Construct a new `CharStrategy` with the parameters it will pass to the"] # [doc = " function underlying `select_char()`."] # [doc = ""] # [doc = " All arguments as per `select_char()`."] pub fn new (special : Cow < 'a , [char] > , preferred : Cow < 'a , [CharRange] > , ranges : Cow < 'a , [CharRange] > ,) -> Self { CharStrategy { special , preferred , ranges , } } # [doc = " Same as `CharStrategy::new()` but using `Cow::Borrowed` for all parts."] pub fn new_borrowed (special : & 'a [char] , preferred : & 'a [CharRange] , ranges : & 'a [CharRange] ,) -> Self { CharStrategy :: new (Cow :: Borrowed (special) , Cow :: Borrowed (preferred) , Cow :: Borrowed (ranges) ,) } }
};
}
