// Generated macro for test (module)
macro_rules! Depcrate_numbertest {
() => {
// Module: crate::number
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn match_for_type_id_should_be_backwards_compatible () { let type_id = kCFNumberFloat32Type ; match type_id { vf64 if vf64 == kCFNumberFloat32Type => assert ! (true) , _ => panic ! ("should not happen") , } ; match type_id { kCFNumberFloat32Type => assert ! (true) , _ => panic ! ("should not happen") , } ; } }
};
}
