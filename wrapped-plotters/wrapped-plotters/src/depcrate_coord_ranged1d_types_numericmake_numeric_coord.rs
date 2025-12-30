// Generated macro for make_numeric_coord (macro)
macro_rules! Depcrate_coord_ranged1d_types_numericmake_numeric_coord {
() => {
// Module: crate::coord::ranged1d::types::numeric
// Provides: {"make_numeric_coord"}
// Dependencies: {}
macro_rules ! make_numeric_coord { ($ type : ty , $ name : ident , $ key_points : ident , $ doc : expr , $ fmt : ident) => { # [doc = $ doc] # [derive (Clone)] pub struct $ name ($ type , $ type) ; impl From < Range <$ type >> for $ name { fn from (range : Range <$ type >) -> Self { return $ name (range . start , range . end) ; } } impl Ranged for $ name { type FormatOption = $ fmt ; type ValueType = $ type ; # [allow (clippy :: float_cmp)] fn map (& self , v : &$ type , limit : (i32 , i32)) -> i32 { if self . 1 == self . 0 { return (limit . 1 - limit . 0) / 2 ; } let logic_length = (* v as f64 - self . 0 as f64) / (self . 1 as f64 - self . 0 as f64) ; let actual_length = limit . 1 - limit . 0 ; if actual_length == 0 { return limit . 1 ; } if logic_length . is_infinite () { if logic_length . is_sign_positive () { return limit . 1 ; } else { return limit . 0 ; } } if actual_length > 0 { return limit . 0 + (actual_length as f64 * logic_length + 1e-3) . floor () as i32 ; } else { return limit . 0 + (actual_length as f64 * logic_length - 1e-3) . ceil () as i32 ; } } fn key_points < Hint : KeyPointHint > (& self , hint : Hint) -> Vec <$ type > { $ key_points ((self . 0 , self . 1) , hint . max_num_points ()) } fn range (& self) -> Range <$ type > { return self . 0 .. self . 1 ; } } } ; ($ type : ty , $ name : ident , $ key_points : ident , $ doc : expr) => { make_numeric_coord ! ($ type , $ name , $ key_points , $ doc , DefaultFormatting) ; } ; }
};
}
