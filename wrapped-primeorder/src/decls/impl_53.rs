macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
        LookupTable!();
        ProjectivePoint!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < C > ProjectivePoint < C > where C : PrimeCurveParams , { # [doc = " Additive identity of the group a.k.a. the point at infinity."] pub const IDENTITY : Self = Self { x : C :: FieldElement :: ZERO , y : C :: FieldElement :: ONE , z : C :: FieldElement :: ZERO , } ; # [doc = " Base point of the curve."] pub const GENERATOR : Self = Self { x : C :: GENERATOR . 0 , y : C :: GENERATOR . 1 , z : C :: FieldElement :: ONE , } ; # [doc = " Returns the affine representation of this point, or `None` if it is the identity."] pub fn to_affine (& self) -> AffinePoint < C > { < C :: FieldElement as Field > :: invert (& self . z) . map (| zinv | self . to_affine_internal (zinv)) . unwrap_or (AffinePoint :: IDENTITY) } pub (super) fn to_affine_internal (self , zinv : C :: FieldElement) -> AffinePoint < C > { AffinePoint { x : self . x * & zinv , y : self . y * & zinv , infinity : 0 , } } # [doc = " Returns `-self`."] pub fn neg (& self) -> Self { Self { x : self . x , y : - self . y , z : self . z , } } # [doc = " Returns `self + other`."] pub fn add (& self , other : & Self) -> Self { C :: PointArithmetic :: add (self , other) } # [doc = " Returns `self + other`."] fn add_mixed (& self , other : & AffinePoint < C >) -> Self { C :: PointArithmetic :: add_mixed (self , other) } # [doc = " Returns `self - other`."] pub fn sub (& self , other : & Self) -> Self { self . add (& other . neg ()) } # [doc = " Returns `self - other`."] fn sub_mixed (& self , other : & AffinePoint < C >) -> Self { self . add_mixed (& other . neg ()) } # [doc = " Returns `[k] self`."] fn mul (& self , k : & Scalar < C >) -> Self where Self : Double , { let mut k = Into :: < C :: Uint > :: into (* k) . to_le_byte_array () ; let mut pc = LookupTable :: new (* self) ; lincomb (array :: from_mut (& mut k) , array :: from_mut (& mut pc)) } }
    };
}

impl_53!()