macro_rules! deps {
    () => {
        Limb!();
        VecType!();
    };
}

macro_rules! large_add {
    () => {
        deps!();
        # [doc = " Add bigint to bigint."] # [inline (always)] pub fn large_add (x : & mut VecType , y : & [Limb]) -> Option < () > { large_add_from (x , y , 0) }
    };
}

large_add!();