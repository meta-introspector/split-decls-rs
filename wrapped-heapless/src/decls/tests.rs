macro_rules! deps {
    () => {
        Vec!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use crate :: Vec ; use embedded_io :: { Error , ErrorKind , Write } ; fn write (w : & mut impl Write , data : & [u8]) -> Result < () , ErrorKind > { w . write_all (data) . map_err (| e | e . kind ()) } # [test] fn test_write () { let mut v : Vec < u8 , 4 > = Vec :: new () ; assert_eq ! (v . len () , 0) ; write (& mut v , & [1 , 2]) . unwrap () ; assert_eq ! (v . len () , 2) ; assert_eq ! (v . as_slice () , & [1 , 2]) ; write (& mut v , & [3]) . unwrap () ; assert_eq ! (v . len () , 3) ; assert_eq ! (v . as_slice () , & [1 , 2 , 3]) ; assert ! (write (& mut v , & [4 , 5]) . is_err ()) ; assert_eq ! (v . len () , 3) ; assert_eq ! (v . as_slice () , & [1 , 2 , 3]) ; } }
    };
}

tests!()