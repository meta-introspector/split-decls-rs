// Generated macro for fields_to_vec (function)
macro_rules! Depcrate_utilfields_to_vec {
() => {
// Module: crate::util
// Provides: {"fields_to_vec"}
// Dependencies: {}
# [doc = " Extract the list of fields from a `Fields` from syn."] # [doc = " We don't care about the style, we always and uniformly use {} in"] # [doc = " struct literal syntax for making struct and enum variant values."] pub fn fields_to_vec (fields : syn :: Fields) -> Vec < syn :: Field > { use syn :: Fields :: * ; match fields { Named (fields) => fields . named . into_iter () . collect () , Unnamed (fields) => fields . unnamed . into_iter () . collect () , Unit => vec ! [] , } }
};
}
