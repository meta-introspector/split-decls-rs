// Generated macro for hashable (function)
macro_rules! Depcrate_com_objecthashable {
() => {
// Module: crate::com_object
// Provides: {"hashable"}
// Dependencies: {}
# [test] fn hashable () { use std :: collections :: HashMap ; let mut map : HashMap < ComObject < MyApp > , & 'static str > = HashMap :: new () ; map . insert (MyApp :: new (100) , "hello") ; map . insert (MyApp :: new (200) , "world") ; }
};
}
