macro_rules! deps {
    () => {
        Integer!();
        Size!();
        AbiAlign!();
        WrappingRange!();
        HasDataLayout!();
        Primitive!();
        Scalar!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Scalar { # [inline] pub fn is_bool (& self) -> bool { use Integer :: * ; matches ! (self , Scalar :: Initialized { value : Primitive :: Int (I8 , false) , valid_range : WrappingRange { start : 0 , end : 1 } }) } # [doc = " Get the primitive representation of this type, ignoring the valid range and whether the"] # [doc = " value is allowed to be undefined (due to being a union)."] pub fn primitive (& self) -> Primitive { match * self { Scalar :: Initialized { value , .. } | Scalar :: Union { value } => value , } } pub fn align (self , cx : & impl HasDataLayout) -> AbiAlign { self . primitive () . align (cx) } pub fn size (self , cx : & impl HasDataLayout) -> Size { self . primitive () . size (cx) } # [inline] pub fn to_union (& self) -> Self { Self :: Union { value : self . primitive () } } # [inline] pub fn valid_range (& self , cx : & impl HasDataLayout) -> WrappingRange { match * self { Scalar :: Initialized { valid_range , .. } => valid_range , Scalar :: Union { value } => WrappingRange :: full (value . size (cx)) , } } # [inline] # [doc = " Allows the caller to mutate the valid range. This operation will panic if attempted on a"] # [doc = " union."] pub fn valid_range_mut (& mut self) -> & mut WrappingRange { match self { Scalar :: Initialized { valid_range , .. } => valid_range , Scalar :: Union { .. } => panic ! ("cannot change the valid range of a union") , } } # [doc = " Returns `true` if all possible numbers are valid, i.e `valid_range` covers the whole"] # [doc = " layout."] # [inline] pub fn is_always_valid < C : HasDataLayout > (& self , cx : & C) -> bool { match * self { Scalar :: Initialized { valid_range , .. } => valid_range . is_full_for (self . size (cx)) , Scalar :: Union { .. } => true , } } # [doc = " Returns `true` if this type can be left uninit."] # [inline] pub fn is_uninit_valid (& self) -> bool { match * self { Scalar :: Initialized { .. } => false , Scalar :: Union { .. } => true , } } # [doc = " Returns `true` if this is a signed integer scalar"] # [inline] pub fn is_signed (& self) -> bool { match self . primitive () { Primitive :: Int (_ , signed) => signed , _ => false , } } }
    };
}

impl_56!()