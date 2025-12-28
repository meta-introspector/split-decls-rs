macro_rules! InsideParensRestConsumed {
    () => {
        type InsideParensRestConsumed < 'a > = (std :: borrow :: Cow < 'a , BStr > , & 'a BStr , usize) ;
    };
}

InsideParensRestConsumed!()