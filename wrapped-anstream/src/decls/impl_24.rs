macro_rules! deps {
    () => {
        VtUtf8Receiver!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl utf8parse :: Receiver for VtUtf8Receiver < '_ > { fn codepoint (& mut self , _ : char) { * self . 0 = true ; } fn invalid_sequence (& mut self) { * self . 0 = true ; } }
    };
}

impl_24!();