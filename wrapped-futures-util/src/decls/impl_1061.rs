macro_rules! impl_1061 {
    () => {
        impl < R > BufReader < R > { delegate_access_inner ! (inner , R , ()) ; # [doc = " Returns a reference to the internally buffered data."] # [doc = ""] # [doc = " Unlike `fill_buf`, this will not attempt to fill the buffer if it is empty."] pub fn buffer (& self) -> & [u8] { & self . buffer [self . pos .. self . cap] } # [doc = " Invalidates all data in the internal buffer."] # [inline] fn discard_buffer (self : Pin < & mut Self >) { let this = self . project () ; * this . pos = 0 ; * this . cap = 0 ; } }
    };
}

impl_1061!()