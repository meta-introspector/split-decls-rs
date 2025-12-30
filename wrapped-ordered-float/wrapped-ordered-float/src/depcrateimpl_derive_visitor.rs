// Generated macro for impl_derive_visitor (module)
macro_rules! Depcrateimpl_derive_visitor {
() => {
// Module: crate
// Provides: {"impl_derive_visitor"}
// Dependencies: {}
# [cfg (feature = "derive-visitor")] mod impl_derive_visitor { use crate :: OrderedFloat ; use derive_visitor :: { Drive , DriveMut , Event , Visitor , VisitorMut } ; impl < T : 'static > Drive for OrderedFloat < T > { fn drive < V : Visitor > (& self , visitor : & mut V) { visitor . visit (self , Event :: Enter) ; visitor . visit (self , Event :: Exit) ; } } impl < T : 'static > DriveMut for OrderedFloat < T > { fn drive_mut < V : VisitorMut > (& mut self , visitor : & mut V) { visitor . visit (self , Event :: Enter) ; visitor . visit (self , Event :: Exit) ; } } # [test] pub fn test_derive_visitor () { # [derive (Debug , Clone , PartialEq , Eq , Drive , DriveMut)] pub enum Literal { Null , Float (OrderedFloat < f64 >) , } # [derive (Visitor , VisitorMut)] # [visitor (Literal (enter))] struct FloatExpr (bool) ; impl FloatExpr { fn enter_literal (& mut self , lit : & Literal) { if let Literal :: Float (_) = lit { self . 0 = true ; } } } assert ! ({ let mut visitor = FloatExpr (false) ; Literal :: Null . drive (& mut visitor) ; ! visitor . 0 }) ; assert ! ({ let mut visitor = FloatExpr (false) ; Literal :: Null . drive_mut (& mut visitor) ; ! visitor . 0 }) ; assert ! ({ let mut visitor = FloatExpr (false) ; Literal :: Float (OrderedFloat (0.0)) . drive (& mut visitor) ; visitor . 0 }) ; assert ! ({ let mut visitor = FloatExpr (false) ; Literal :: Float (OrderedFloat (0.0)) . drive_mut (& mut visitor) ; visitor . 0 }) ; } }
};
}
