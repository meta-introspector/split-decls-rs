macro_rules! panic_does_not_fit {
    () => {
        # [cold] fn panic_does_not_fit (size : usize , nbytes : usize) -> ! { panic ! ("size too large: the integer type can fit {} bytes, but nbytes is {}" , size , nbytes) ; }
    };
}

panic_does_not_fit!()