macro_rules! hi {
    () => {
        # [doc = " Extract the hi bits from the buffer."] macro_rules ! hi { (@ 1 $ self : ident , $ rview : ident , $ t : ident , $ fn : ident) => { { $ fn ($ rview [0] as $ t) } } ; (@ 2 $ self : ident , $ rview : ident , $ t : ident , $ fn : ident) => { { let r0 = $ rview [0] as $ t ; let r1 = $ rview [1] as $ t ; $ fn (r0 , r1) } } ; (@ nonzero2 $ self : ident , $ rview : ident , $ t : ident , $ fn : ident) => { { let (v , n) = hi ! (@ 2 $ self , $ rview , $ t , $ fn) ; (v , n || nonzero ($ self , 2)) } } ; (@ 3 $ self : ident , $ rview : ident , $ t : ident , $ fn : ident) => { { let r0 = $ rview [0] as $ t ; let r1 = $ rview [1] as $ t ; let r2 = $ rview [2] as $ t ; $ fn (r0 , r1 , r2) } } ; (@ nonzero3 $ self : ident , $ rview : ident , $ t : ident , $ fn : ident) => { { let (v , n) = hi ! (@ 3 $ self , $ rview , $ t , $ fn) ; (v , n || nonzero ($ self , 3)) } } ; }
    };
}

hi!();