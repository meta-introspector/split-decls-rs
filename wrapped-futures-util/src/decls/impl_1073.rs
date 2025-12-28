macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! impl_1073 {
    () => {
        deps!();
        impl < W > BufWriter < W > { delegate_access_inner ! (inner , W , ()) ; # [doc = " Returns a reference to the internally buffered data."] pub fn buffer (& self) -> & [u8] { & self . buf } # [doc = " Capacity of `buf`. how many chars can be held in buffer"] pub (super) fn capacity (& self) -> usize { self . buf . capacity () } # [doc = " Remaining number of bytes to reach `buf` 's capacity"] # [inline] pub (super) fn spare_capacity (& self) -> usize { self . buf . capacity () - self . buf . len () } # [doc = " Write a byte slice directly into buffer"] # [doc = ""] # [doc = " Will truncate the number of bytes written to `spare_capacity()` so you want to"] # [doc = " calculate the size of your slice to avoid losing bytes"] # [doc = ""] # [doc = " Based on `std::io::BufWriter`"] pub (super) fn write_to_buf (self : Pin < & mut Self > , buf : & [u8]) -> usize { let available = self . spare_capacity () ; let amt_to_buffer = available . min (buf . len ()) ; unsafe { self . write_to_buffer_unchecked (& buf [.. amt_to_buffer]) ; } amt_to_buffer } # [doc = " Write byte slice directly into `self.buf`"] # [doc = ""] # [doc = " Based on `std::io::BufWriter`"] # [inline] unsafe fn write_to_buffer_unchecked (self : Pin < & mut Self > , buf : & [u8]) { debug_assert ! (buf . len () <= self . spare_capacity ()) ; let this = self . project () ; let old_len = this . buf . len () ; let buf_len = buf . len () ; let src = buf . as_ptr () ; unsafe { let dst = this . buf . as_mut_ptr () . add (old_len) ; ptr :: copy_nonoverlapping (src , dst , buf_len) ; this . buf . set_len (old_len + buf_len) ; } } }
    };
}

impl_1073!()