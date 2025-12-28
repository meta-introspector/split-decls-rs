macro_rules! deps {
    () => {
        Chain!();
        BufMut!();
        UninitSlice!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        unsafe impl < T , U > BufMut for Chain < T , U > where T : BufMut , U : BufMut , { fn remaining_mut (& self) -> usize { self . a . remaining_mut () . saturating_add (self . b . remaining_mut ()) } fn chunk_mut (& mut self) -> & mut UninitSlice { if self . a . has_remaining_mut () { self . a . chunk_mut () } else { self . b . chunk_mut () } } unsafe fn advance_mut (& mut self , mut cnt : usize) { let a_rem = self . a . remaining_mut () ; if a_rem != 0 { if a_rem >= cnt { self . a . advance_mut (cnt) ; return ; } self . a . advance_mut (a_rem) ; cnt -= a_rem ; } self . b . advance_mut (cnt) ; } }
    };
}

impl_25!();