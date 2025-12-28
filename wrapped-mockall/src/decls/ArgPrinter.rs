macro_rules! ArgPrinter {
    () => {
        # [doc (hidden)] pub struct ArgPrinter < 'a , T > (pub & 'a T) ;
    };
}

ArgPrinter!();