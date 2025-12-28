macro_rules! deps {
    () => {
        InlayHintLabelBuilder!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl fmt :: Write for InlayHintLabelBuilder < '_ > { fn write_str (& mut self , s : & str) -> fmt :: Result { self . last_part . write_str (s) } }
    };
}

impl_279!()