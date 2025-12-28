macro_rules! deps {
    () => {
        DepthCalculate!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < 'a > DepthCalculate < 'a > { pub fn new (max_depth : & 'a mut usize) -> Self { Self { max_depth , current_depth : 0 , } } }
    };
}

impl_319!();