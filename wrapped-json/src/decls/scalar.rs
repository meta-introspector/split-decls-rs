macro_rules! deps {
    () => {
        Limb!();
        Wide!();
    };
}

macro_rules! scalar {
    () => {
        deps!();
        mod scalar { use super :: * ; # [doc = " Add two small integers and return the resulting value and if overflow happens."] # [inline] pub fn add (x : Limb , y : Limb) -> (Limb , bool) { x . overflowing_add (y) } # [doc = " AddAssign two small integers and return if overflow happens."] # [inline] pub fn iadd (x : & mut Limb , y : Limb) -> bool { let t = add (* x , y) ; * x = t . 0 ; t . 1 } # [doc = " Subtract two small integers and return the resulting value and if overflow happens."] # [inline] pub fn sub (x : Limb , y : Limb) -> (Limb , bool) { x . overflowing_sub (y) } # [doc = " SubAssign two small integers and return if overflow happens."] # [inline] pub fn isub (x : & mut Limb , y : Limb) -> bool { let t = sub (* x , y) ; * x = t . 0 ; t . 1 } # [doc = " Multiply two small integers (with carry) (and return the overflow contribution)."] # [doc = ""] # [doc = " Returns the (low, high) components."] # [inline] pub fn mul (x : Limb , y : Limb , carry : Limb) -> (Limb , Limb) { let z : Wide = as_wide (x) * as_wide (y) + as_wide (carry) ; let bits = mem :: size_of :: < Limb > () * 8 ; (as_limb (z) , as_limb (z >> bits)) } # [doc = " Multiply two small integers (with carry) (and return if overflow happens)."] # [inline] pub fn imul (x : & mut Limb , y : Limb , carry : Limb) -> Limb { let t = mul (* x , y , carry) ; * x = t . 0 ; t . 1 } }
    };
}

scalar!();