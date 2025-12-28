macro_rules! deps {
    () => {
        Limb!();
        Wide!();
    };
}

macro_rules! scalar_mul {
    () => {
        deps!();
        # [doc = " Multiply two small integers (with carry) (and return the overflow contribution)."] # [doc = ""] # [doc = " Returns the (low, high) components."] # [inline (always)] pub fn scalar_mul (x : Limb , y : Limb , carry : Limb) -> (Limb , Limb) { let z : Wide = (x as Wide) * (y as Wide) + (carry as Wide) ; (z as Limb , (z >> LIMB_BITS) as Limb) }
    };
}

scalar_mul!()