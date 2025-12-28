macro_rules! error_scale {
    () => {
        # [doc = " Get the full error scale."] # [inline (always)] const fn error_scale () -> u32 { 8 }
    };
}

error_scale!();