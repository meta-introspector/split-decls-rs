macro_rules! deps {
    () => {
        Limb!();
        VecType!();
    };
}

macro_rules! small_add {
    () => {
        deps!();
        # [doc = " Add small integer to bigint."] # [inline (always)] pub fn small_add (x : & mut VecType , y : Limb) -> Option < () > { small_add_from (x , y , 0) }
    };
}

small_add!();