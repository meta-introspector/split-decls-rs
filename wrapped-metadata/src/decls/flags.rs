macro_rules! flags {
    () => {
        macro_rules ! flags { ($ name : ident , $ size : ty) => { # [derive (Default , Copy , Clone , PartialEq , Eq , Debug , Ord , PartialOrd)] pub struct $ name (pub (crate) $ size) ; impl $ name { pub fn contains (& self , contains : Self) -> bool { * self & contains == contains } } impl std :: ops :: BitOr for $ name { type Output = Self ; fn bitor (self , other : Self) -> Self { Self (self . 0 | other . 0) } } impl std :: ops :: BitAnd for $ name { type Output = Self ; fn bitand (self , other : Self) -> Self { Self (self . 0 & other . 0) } } impl std :: ops :: BitOrAssign for $ name { fn bitor_assign (& mut self , other : Self) { self . 0 . bitor_assign (other . 0) } } impl std :: ops :: BitAndAssign for $ name { fn bitand_assign (& mut self , other : Self) { self . 0 . bitand_assign (other . 0) } } impl std :: ops :: Not for $ name { type Output = Self ; fn not (self) -> Self { Self (self . 0 . not ()) } } } ; }
    };
}

flags!()