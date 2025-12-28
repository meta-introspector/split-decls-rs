macro_rules! escape {
    () => {
        macro_rules ! escape { ($ ($ inner : expr) ,*) => { concat ! ("\x1B[" , $ ($ inner) ,*, "m") } ; }
    };
}

escape!()