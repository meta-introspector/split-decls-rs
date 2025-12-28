macro_rules! deps {
    () => {
        Convert!();
    };
}

macro_rules! convert_primitive_bytes {
    () => {
        deps!();
        macro_rules ! convert_primitive_bytes { ($ a : ty , $ b : ty) => { impl Convert <$ b > for $ a { # [inline (always)] fn convert (self) -> $ b { self . to_ne_bytes () } } impl Convert <$ a > for $ b { # [inline (always)] fn convert (self) -> $ a { <$ a >:: from_ne_bytes (self) } } } ; }
    };
}

convert_primitive_bytes!()