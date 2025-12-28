macro_rules! deps {
    () => {
        Rng!();
        SliceRandom!();
        IncreasingUniform!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < T > SliceRandom for [T] { fn shuffle < R > (& mut self , rng : & mut R) where R : Rng + ? Sized , { if self . len () <= 1 { return ; } self . partial_shuffle (rng , self . len ()) ; } fn partial_shuffle < R > (& mut self , rng : & mut R , amount : usize) -> (& mut [T] , & mut [T]) where R : Rng + ? Sized , { let m = self . len () . saturating_sub (amount) ; if self . len () < (u32 :: MAX as usize) { let mut chooser = IncreasingUniform :: new (rng , m as u32) ; for i in m .. self . len () { let index = chooser . next_index () ; self . swap (i , index) ; } } else { for i in m .. self . len () { let index = rng . random_range (.. i + 1) ; self . swap (i , index) ; } } let r = self . split_at_mut (m) ; (r . 1 , r . 0) } }
    };
}

impl_290!()