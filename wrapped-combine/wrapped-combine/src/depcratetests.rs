// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: parser :: char :: { char , string } ; use super :: * ; # [test] fn chainl1_error_consume () { fn first < T , U > (t : T , _ : U) -> T { t } let mut p = chainl1 (string ("abc") , char (',') . map (| _ | first)) ; assert ! (p . parse ("abc,ab") . is_err ()) ; } # [test] fn choice_strings () { let mut fruits = [attempt (string ("Apple")) , attempt (string ("Banana")) , attempt (string ("Cherry")) , attempt (string ("Date")) , attempt (string ("Fig")) , attempt (string ("Grape")) ,] ; let mut parser = choice (& mut fruits) ; assert_eq ! (parser . parse ("Apple") , Ok (("Apple" , ""))) ; assert_eq ! (parser . parse ("Banana") , Ok (("Banana" , ""))) ; assert_eq ! (parser . parse ("Cherry") , Ok (("Cherry" , ""))) ; assert_eq ! (parser . parse ("DateABC") , Ok (("Date" , "ABC"))) ; assert_eq ! (parser . parse ("Fig123") , Ok (("Fig" , "123"))) ; assert_eq ! (parser . parse ("GrapeApple") , Ok (("Grape" , "Apple"))) ; } }
};
}
