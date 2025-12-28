macro_rules! deps {
    () => {
        NonZero!();
        MulMod!();
        SquareRoot!();
        Limb!();
        Monty!();
        RemLimb!();
        SquareMod!();
        NegMod!();
        BitOps!();
        AddMod!();
        DivRemLimb!();
        SubMod!();
        Integer!();
    };
}

macro_rules! Unsigned {
    () => {
        deps!();
        # [doc = " Unsigned [`Integer`]s."] pub trait Unsigned : AddMod < Output = Self > + BitOps + Div < NonZero < Self > , Output = Self > + for < 'a > Div < & 'a NonZero < Self > , Output = Self > + DivRemLimb + From < u8 > + From < u16 > + From < u32 > + From < u64 > + From < Limb > + Integer + MulMod < Output = Self > + NegMod < Output = Self > + RemLimb + SquareRoot + SquareMod < Output = Self > + SubMod < Output = Self > { # [doc = " The corresponding Montgomery representation,"] # [doc = " optimized for the performance of modular operations at the price of a conversion overhead."] type Monty : Monty < Integer = Self > ; # [doc = " Returns an integer with the first limb set to `limb`, and the same precision as `other`."] fn from_limb_like (limb : Limb , other : & Self) -> Self ; }
    };
}

Unsigned!()