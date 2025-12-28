macro_rules! Point {
    () => {
        # [allow (dead_code)] # [derive (Debug , Eq , PartialEq)] struct Point { x : i32 , y : i32 , z : i32 , }
    };
}

Point!();