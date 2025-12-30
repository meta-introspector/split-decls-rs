// Generated macro for diff_list (macro)
macro_rules! Depcrate_testsdiff_list {
() => {
// Module: crate::tests
// Provides: {"diff_list"}
// Dependencies: {}
macro_rules ! diff_list { () => { Solution { text1 : Range :: empty () , text2 : Range :: empty () , diffs : Vec :: new () , } } ; ($ ($ kind : ident ($ text : literal)) ,+ $ (,) ?) => { { # [allow (unused_macro_rules)] macro_rules ! text1 { (Insert , $ s : literal) => { "" } ; (Delete , $ s : literal) => { $ s } ; (Equal , $ s : literal) => { $ s } ; } # [allow (unused_macro_rules)] macro_rules ! text2 { (Insert , $ s : literal) => { $ s } ; (Delete , $ s : literal) => { "" } ; (Equal , $ s : literal) => { $ s } ; } let text1 = range ! (concat ! ($ (text1 ! ($ kind , $ text)) ,*)) ; let text2 = range ! (concat ! ($ (text2 ! ($ kind , $ text)) ,*)) ; let (_i , _j) = (& mut 0 , & mut 0) ; # [allow (unused_macro_rules)] macro_rules ! range { (Insert , $ s : literal) => { Diff :: Insert (range (text2 . doc , _j , $ s)) } ; (Delete , $ s : literal) => { Diff :: Delete (range (text1 . doc , _i , $ s)) } ; (Equal , $ s : literal) => { Diff :: Equal (range (text1 . doc , _i , $ s) , range (text2 . doc , _j , $ s)) } ; } Solution { text1 , text2 , diffs : vec ! [$ (range ! ($ kind , $ text)) ,*] , } } } ; }
};
}
