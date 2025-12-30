// Generated macro for impl_96 (impl)
macro_rules! Depcrate_astimpl_96 {
() => {
// Module: crate::ast
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'a > TryFrom < Pair < 'a , Rule > > for CompassPt { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { match p . clone () . into_inner () . next () . ok_or (ParseError :: missing_pair (p , vec ! [Rule :: n , Rule :: ne , Rule :: e , Rule :: se , Rule :: s , Rule :: sw , Rule :: w , Rule :: nw , Rule :: c , Rule :: underscore ,] ,)) ? . as_rule () { Rule :: n => Ok (CompassPt :: N) , Rule :: ne => Ok (CompassPt :: NE) , Rule :: e => Ok (CompassPt :: E) , Rule :: se => Ok (CompassPt :: SE) , Rule :: s => Ok (CompassPt :: S) , Rule :: sw => Ok (CompassPt :: SW) , Rule :: w => Ok (CompassPt :: W) , Rule :: nw => Ok (CompassPt :: NW) , Rule :: c => Ok (CompassPt :: C) , Rule :: underscore => Ok (CompassPt :: Underscore) , r => Err (ParseError :: expect_rule (vec ! [Rule :: n , Rule :: ne , Rule :: e , Rule :: se , Rule :: s , Rule :: sw , Rule :: w , Rule :: nw , Rule :: c , Rule :: underscore ,] , r ,)) , } } }
};
}
