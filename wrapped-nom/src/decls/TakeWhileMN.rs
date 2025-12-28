macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! TakeWhileMN {
    () => {
        deps!();
        # [doc = " Parser implementation for [take_while_m_n]"] pub struct TakeWhileMN < F , E > { m : usize , n : usize , predicate : F , e : PhantomData < E > , }
    };
}

TakeWhileMN!();