macro_rules! deps {
    () => {
        Zero!();
        CheckedMul!();
        Limb!();
        NonZero!();
        CheckedSub!();
        ConstantTimeSelect!();
        One!();
        ShlVartime!();
        CheckedDiv!();
        ShrVartime!();
        CheckedAdd!();
    };
}

macro_rules! Integer {
    () => {
        deps!();
        # [doc = " Integer trait: represents common functionality of integer types provided by this crate."] pub trait Integer : 'static + Add < Output = Self > + for < 'a > Add < & 'a Self , Output = Self > + AddAssign < Self > + for < 'a > AddAssign < & 'a Self > + AsRef < [Limb] > + BitAnd < Output = Self > + for < 'a > BitAnd < & 'a Self , Output = Self > + BitAndAssign + for < 'a > BitAndAssign < & 'a Self > + BitOr < Output = Self > + for < 'a > BitOr < & 'a Self , Output = Self > + BitOrAssign + for < 'a > BitOrAssign < & 'a Self > + BitXor < Output = Self > + for < 'a > BitXor < & 'a Self , Output = Self > + BitXorAssign + for < 'a > BitXorAssign < & 'a Self > + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + Clone + ConstantTimeEq + ConstantTimeGreater + ConstantTimeLess + ConstantTimeSelect + Debug + Default + DivAssign < NonZero < Self > > + for < 'a > DivAssign < & 'a NonZero < Self > > + Eq + fmt :: LowerHex + fmt :: UpperHex + fmt :: Binary + Mul < Output = Self > + for < 'a > Mul < & 'a Self , Output = Self > + MulAssign < Self > + for < 'a > MulAssign < & 'a Self > + Not < Output = Self > + One + Ord + Rem < NonZero < Self > , Output = Self > + for < 'a > Rem < & 'a NonZero < Self > , Output = Self > + RemAssign < NonZero < Self > > + for < 'a > RemAssign < & 'a NonZero < Self > > + Send + Sized + Shl < u32 , Output = Self > + ShlAssign < u32 > + ShlVartime + Shr < u32 , Output = Self > + ShrAssign < u32 > + ShrVartime + Sub < Output = Self > + for < 'a > Sub < & 'a Self , Output = Self > + SubAssign < Self > + for < 'a > SubAssign < & 'a Self > + Sync + WrappingAdd + WrappingSub + WrappingMul + WrappingNeg + WrappingShl + WrappingShr + Zero { # [doc = " Borrow the raw limbs used to represent this integer."] fn as_limbs (& self) -> & [Limb] ; # [doc = " Mutably borrow the raw limbs used to represent this integer."] fn as_mut_limbs (& mut self) -> & mut [Limb] ; # [doc = " Number of limbs in this integer."] fn nlimbs (& self) -> usize ; # [doc = " Is this integer value an odd number?"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " If odd, returns `Choice(1)`. Otherwise, returns `Choice(0)`."] fn is_odd (& self) -> Choice { self . as_ref () . first () . map (| limb | limb . is_odd ()) . unwrap_or_else (| | Choice :: from (0)) } # [doc = " Is this integer value an even number?"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " If even, returns `Choice(1)`. Otherwise, returns `Choice(0)`."] fn is_even (& self) -> Choice { ! self . is_odd () } }
    };
}

Integer!()