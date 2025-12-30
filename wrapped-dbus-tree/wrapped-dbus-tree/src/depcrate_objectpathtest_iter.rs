// Generated macro for test_iter (function)
macro_rules! Depcrate_objectpathtest_iter {
() => {
// Module: crate::objectpath
// Provides: {"test_iter"}
// Dependencies: {}
# [test] fn test_iter () { let f = super :: Factory :: new_fn :: < () > () ; let t = f . tree (()) . add (f . object_path ("/echo" , ()) . introspectable () . add (f . interface ("com.example.echo" , ()) . add_m (f . method ("Echo" , () , | _ | unimplemented ! ()) . in_arg (("request" , "s")) . out_arg (("reply" , "s"))) . add_p (f . property :: < i32 , _ > ("EchoCount" , ())) . add_s (f . signal ("Echoed" , ()) . arg (("data" , "s")) . deprecated ()))) . add (f . object_path ("/echo/subpath" , ())) ; let paths : Vec < _ > = t . iter () . collect () ; assert_eq ! (paths . len () , 2) ; }
};
}
