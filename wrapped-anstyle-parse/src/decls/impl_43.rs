macro_rules! deps {
    () => {
        VtUtf8Receiver!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        # [cfg (feature = "utf8")] impl utf8 :: Receiver for VtUtf8Receiver < '_ > { fn codepoint (& mut self , c : char) { * self . 0 = Some (c) ; } fn invalid_sequence (& mut self) { * self . 0 = Some ('�') ; } }
    };
}

impl_43!();