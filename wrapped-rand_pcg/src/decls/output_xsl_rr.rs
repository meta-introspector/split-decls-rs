macro_rules! output_xsl_rr {
    () => {
        # [inline (always)] fn output_xsl_rr (state : u128) -> u64 { const XSHIFT : u32 = 64 ; const ROTATE : u32 = 122 ; let rot = (state >> ROTATE) as u32 ; let xsl = ((state >> XSHIFT) as u64) ^ (state as u64) ; xsl . rotate_right (rot) }
    };
}

output_xsl_rr!()