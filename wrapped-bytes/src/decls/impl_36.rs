macro_rules! deps {
    () => {
        UninitSlice!();
        BufMut!();
        Limit!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        unsafe impl < T : BufMut > BufMut for Limit < T > { fn remaining_mut (& self) -> usize { cmp :: min (self . inner . remaining_mut () , self . limit) } fn chunk_mut (& mut self) -> & mut UninitSlice { let bytes = self . inner . chunk_mut () ; let end = cmp :: min (bytes . len () , self . limit) ; & mut bytes [.. end] } unsafe fn advance_mut (& mut self , cnt : usize) { assert ! (cnt <= self . limit) ; self . inner . advance_mut (cnt) ; self . limit -= cnt ; } }
    };
}

impl_36!();