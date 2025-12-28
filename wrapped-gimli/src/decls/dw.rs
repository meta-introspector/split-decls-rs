macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! dw {
    () => {
        deps!();
        macro_rules ! dw { ($ (# [$ meta : meta]) * $ struct_name : ident ($ struct_type : ty) { $ ($ name : ident = $ val : expr) ,+ $ (,) ? } $ (, aliases { $ ($ alias_name : ident = $ alias_val : expr) ,+ $ (,) ? }) ?) => { $ (# [$ meta]) * # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct $ struct_name (pub $ struct_type) ; $ (pub const $ name : $ struct_name = $ struct_name ($ val) ;) + $ ($ (pub const $ alias_name : $ struct_name = $ struct_name ($ alias_val) ;) +) * impl $ struct_name { pub fn static_string (& self) -> Option <&'static str > { Some (match * self { $ ($ name => stringify ! ($ name) ,) + _ => return None , }) } } impl fmt :: Display for $ struct_name { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> Result < () , fmt :: Error > { if let Some (s) = self . static_string () { f . pad (s) } else { # [cfg (feature = "read")] { f . pad (& format ! ("Unknown {}: {}" , stringify ! ($ struct_name) , self . 0)) } # [cfg (not (feature = "read"))] { write ! (f , "Unknown {}: {}" , stringify ! ($ struct_name) , self . 0) } } } } } ; }
    };
}

dw!()