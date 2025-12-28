macro_rules! deps {
    () => {
        Item!();
        Match!();
        Matcher!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < 'a > Matcher < 'a > { # [doc = " Match the lefthand-side `item` against this spec and return `(true, Some<rhs>)` to gain the other,"] # [doc = " transformed righthand-side of the match as configured by the refspec."] # [doc = " Or return `(true, None)` if there was no `rhs` but the `item` matched."] # [doc = " Lastly, return `(false, None)` if `item` didn't match at all."] # [doc = ""] # [doc = " This may involve resolving a glob with an allocation, as the destination is built using the matching portion of a glob."] pub fn matches_lhs (& self , item : Item < '_ >) -> (bool , Option < Cow < 'a , BStr > >) { match (self . lhs , self . rhs) { (Some (lhs) , None) => (lhs . matches (item) . is_match () , None) , (Some (lhs) , Some (rhs)) => lhs . matches (item) . into_match_outcome (rhs , item) , (None , _) => (false , None) , } } # [doc = " Match the righthand-side `item` against this spec and return `(true, Some<lhs>)` to gain the other,"] # [doc = " transformed lefthand-side of the match as configured by the refspec."] # [doc = " Or return `(true, None)` if there was no `lhs` but the `item` matched."] # [doc = " Lastly, return `(false, None)` if `item` didn't match at all."] # [doc = ""] # [doc = " This may involve resolving a glob with an allocation, as the destination is built using the matching portion of a glob."] pub fn matches_rhs (& self , item : Item < '_ >) -> (bool , Option < Cow < 'a , BStr > >) { match (self . lhs , self . rhs) { (None , Some (rhs)) => (rhs . matches (item) . is_match () , None) , (Some (lhs) , Some (rhs)) => rhs . matches (item) . into_match_outcome (lhs , item) , (_ , None) => (false , None) , } } }
    };
}

impl_43!()