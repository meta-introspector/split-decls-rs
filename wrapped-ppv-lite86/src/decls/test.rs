macro_rules! deps {
    () => {
        Machine!();
        GenericMachine!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [cfg (test)] mod test { use super :: * ; # [test] fn test_bswap32 () { let xs = [0x0f0e_0d0c , 0x0b0a_0908 , 0x0706_0504 , 0x0302_0100] ; let ys = [0x0c0d_0e0f , 0x0809_0a0b , 0x0405_0607 , 0x0001_0203] ; let m = unsafe { GenericMachine :: instance () } ; let x : < GenericMachine as Machine > :: u32x4 = m . vec (xs) ; let x = x . bswap () ; let y = m . vec (ys) ; assert_eq ! (x , y) ; } }
    };
}

test!();