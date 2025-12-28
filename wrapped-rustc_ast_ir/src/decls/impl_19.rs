macro_rules! deps {
    () => {
        Mutability!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Mutability { pub fn invert (self) -> Self { match self { Mutability :: Mut => Mutability :: Not , Mutability :: Not => Mutability :: Mut , } } # [doc = " Returns `\"\"` (empty string) or `\"mut \"` depending on the mutability."] pub fn prefix_str (self) -> & 'static str { match self { Mutability :: Mut => "mut " , Mutability :: Not => "" , } } # [doc = " Returns `\"&\"` or `\"&mut \"` depending on the mutability."] pub fn ref_prefix_str (self) -> & 'static str { match self { Mutability :: Not => "&" , Mutability :: Mut => "&mut " , } } # [doc = " Returns `\"const\"` or `\"mut\"` depending on the mutability."] pub fn ptr_str (self) -> & 'static str { match self { Mutability :: Not => "const" , Mutability :: Mut => "mut" , } } # [doc = " Returns `\"\"` (empty string) or `\"mutably \"` depending on the mutability."] pub fn mutably_str (self) -> & 'static str { match self { Mutability :: Not => "" , Mutability :: Mut => "mutably " , } } # [doc = " Return `true` if self is mutable"] pub fn is_mut (self) -> bool { matches ! (self , Self :: Mut) } # [doc = " Return `true` if self is **not** mutable"] pub fn is_not (self) -> bool { matches ! (self , Self :: Not) } }
    };
}

impl_19!();