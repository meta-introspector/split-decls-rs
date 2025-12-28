macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! panic_oob {
    () => {
        deps!();
        macro_rules ! panic_oob { ($ method_name : expr , $ index : expr , $ len : expr) => { panic ! (concat ! ("ArrayVec::" , $ method_name , ": index {} is out of bounds in vector of length {}") , $ index , $ len) } ; }
    };
}

panic_oob!()