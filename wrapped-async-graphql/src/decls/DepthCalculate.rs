macro_rules! DepthCalculate {
    () => {
        pub struct DepthCalculate < 'a > { max_depth : & 'a mut usize , current_depth : usize , }
    };
}

DepthCalculate!();