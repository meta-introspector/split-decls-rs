macro_rules! add_fcheck {
    () => {
        # [doc = " Generate FCHECK from CMF and FLG (without FCKECH )so that they are correct according to the"] # [doc = " specification, i.e (CMF*256 + FCHK) % 31 = 0."] # [doc = " Returns flg with the FCHKECK bits added (any existing FCHECK bits are ignored)."] # [inline] fn add_fcheck (cmf : u8 , flg : u8) -> u8 { let rem = ((usize :: from (cmf) * 256) + usize :: from (flg)) % usize :: from (FCHECK_DIVISOR) ; let flg = flg & 0b11100000 ; flg + (FCHECK_DIVISOR - rem as u8) }
    };
}

add_fcheck!()