// Generated macro for iter_by_import (function)
macro_rules! Depcrate_jsiter_by_import {
() => {
// Module: crate::js
// Provides: {"iter_by_import"}
// Dependencies: {}
# [doc = " Iterate over the imports in a deterministic order."] fn iter_by_import < 'a , T > (map : & 'a HashMap < ImportId , T > , module : & Module ,) -> Vec < (& 'a ImportId , & 'a T) > { let mut items : Vec < _ > = map . iter () . collect () ; items . sort_by (| & (a , _) , & (b , _) | { let a = module . imports . get (* a) ; let b = module . imports . get (* b) ; a . name . cmp (& b . name) }) ; items }
};
}
