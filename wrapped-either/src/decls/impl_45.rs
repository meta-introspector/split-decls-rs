macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        # [cfg (any (test , feature = "std"))] # [doc = " Requires crate feature `\"std\"`"] impl < L , R > BufRead for Either < L , R > where L : BufRead , R : BufRead , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { for_both ! (self , inner => inner . fill_buf ()) } fn consume (& mut self , amt : usize) { for_both ! (self , inner => inner . consume (amt)) } fn read_until (& mut self , byte : u8 , buf : & mut std :: vec :: Vec < u8 >) -> io :: Result < usize > { for_both ! (self , inner => inner . read_until (byte , buf)) } fn read_line (& mut self , buf : & mut std :: string :: String) -> io :: Result < usize > { for_both ! (self , inner => inner . read_line (buf)) } }
    };
}

impl_45!();