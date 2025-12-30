// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn it_works () { let x = transform ("Hello World" , false , true) ; assert_eq ! (x , "Ħeeŀŀoo Ẇoořŀḓ") ; let x = transform ("Hello World" , false , false) ; assert_eq ! (x , "Ħeŀŀo Ẇořŀḓ") ; let x = transform ("Hello World" , true , false) ; assert_eq ! (x , "Hǝʅʅo Moɹʅp") ; let x = transform ("f" , false , true) ; assert_eq ! (x , "ƒ") ; } # [test] fn dom_test () { let x = transform_dom ("Hello <a>World</a>" , false , true , false) ; assert_eq ! (x , "Ħeeŀŀoo <a>Ẇoořŀḓ</a>") ; let x = transform_dom ("Hello <a>World</a> in <b>my</b> House." , false , true , false) ; assert_eq ! (x , "Ħeeŀŀoo <a>Ẇoořŀḓ</a> iƞ <b>ḿẏ</b> Ħoouuşee.") ; let x = transform_dom ("Hello World within markers" , false , false , true) ; assert_eq ! (x , "[Ħeŀŀo Ẇořŀḓ ẇiŧħiƞ ḿařķeřş]") ; let x = transform_dom ("f" , false , true , false) ; assert_eq ! (x , "f") ; } }
};
}
