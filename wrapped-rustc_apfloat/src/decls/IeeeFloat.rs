macro_rules! deps {
    () => {
        ExpInt!();
        Float!();
        Semantics!();
        Limb!();
        Category!();
    };
}

macro_rules! IeeeFloat {
    () => {
        deps!();
        # [doc = " A floating point number that uses IEEE semantics."] # [doc = ""] # [doc = " Usually you will want to use the available type aliases of this type"] # [doc = " (e.g., [`Single`], [`Double`]) rather than referencing it directly."] # [doc = ""] # [doc = " If `S` implements [`Semantics`], this type will implement [`Float`]."] # [must_use] pub struct IeeeFloat < S > { # [doc = " Absolute significand value (including the integer bit)."] sig : [Limb ; 1] , # [doc = " The signed unbiased exponent of the value."] exp : ExpInt , # [doc = " What kind of floating point number this is."] read_only_category_do_not_mutate : Category , # [doc = " Sign bit of the number."] read_only_sign_do_not_mutate : bool , marker : PhantomData < S > , }
    };
}

IeeeFloat!();