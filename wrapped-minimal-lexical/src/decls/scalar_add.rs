macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! scalar_add {
    () => {
        deps!();
        # [doc = " Add two small integers and return the resulting value and if overflow happens."] # [inline (always)] pub fn scalar_add (x : Limb , y : Limb) -> (Limb , bool) { x . overflowing_add (y) }
    };
}

scalar_add!()