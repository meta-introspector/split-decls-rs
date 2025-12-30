// Generated macro for take_zip_from_parts (function)
macro_rules! Depcrate_futuretake_zip_from_parts {
() => {
// Module: crate::future
// Provides: {"take_zip_from_parts"}
// Dependencies: {}
# [doc = " Extracts the contents of two options and zips them, handling `(Some(_), None)` cases"] fn take_zip_from_parts < T1 , T2 > (o1 : & mut Option < T1 > , o2 : & mut Option < T2 >) -> Poll < (T1 , T2) > { match (o1 . take () , o2 . take ()) { (Some (t1) , Some (t2)) => Poll :: Ready ((t1 , t2)) , (o1x , o2x) => { * o1 = o1x ; * o2 = o2x ; Poll :: Pending } } }
};
}
