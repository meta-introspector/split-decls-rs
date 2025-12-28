macro_rules! count {
    () => {
        # [doc = " A unit for displaying human readable numbers with throughput and progress percentage, and a single decimal place."] pub fn count (name : & 'static str) -> Option < Unit > { count_with_decimals (name , 1) }
    };
}

count!()