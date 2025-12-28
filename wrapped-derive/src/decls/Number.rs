macro_rules! Number {
    () => {
        # [derive (Clone)] pub enum Number { F64 (f64) , I64 (i64) , }
    };
}

Number!();