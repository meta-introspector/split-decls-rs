macro_rules! error_halfscale {
    () => {
        # [doc = " Get the half error scale."] # [inline (always)] const fn error_halfscale () -> u32 { error_scale () / 2 }
    };
}

error_halfscale!();