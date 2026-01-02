mkuse!{use derive_where :: derive_where ;}
mkuse!{# [cfg (feature = "nightly")] use rustc_macros :: { Decodable_NoContext , Encodable_NoContext , HashStable_NoContext } ;}
mkuse!{use crate :: Interner ;}
mkitem!{mkenum!{# [derive_where (Clone , Copy , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum GenericArgKind < I : Interner > { Lifetime (I :: Region) , Type (I :: Ty) , Const (I :: Const) , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for GenericArgKind < I > { }}}
mkitem!{mkenum!{# [derive_where (Clone , Copy , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum TermKind < I : Interner > { Ty (I :: Ty) , Const (I :: Const) , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for TermKind < I > { }}}