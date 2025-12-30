// Generated macro for fn_customdata (function)
macro_rules! Depcrate_factoryfn_customdata {
() => {
// Module: crate::factory
// Provides: {"fn_customdata"}
// Dependencies: {}
# [test] fn fn_customdata () { # [derive (Default)] struct Custom ; impl DataType for Custom { type Tree = () ; type ObjectPath = Arc < u8 > ; type Interface = () ; type Property = () ; type Method = i32 ; type Signal = () ; } let f = Factory :: new_fn :: < Custom > () ; let m = f . method ("test" , 789 , | _ | unimplemented ! ()) ; assert_eq ! (* m . get_data () , 789) ; let o = f . object_path ("/test/test" , Arc :: new (7)) ; assert_eq ! (** o . get_data () , 7) ; }
};
}
